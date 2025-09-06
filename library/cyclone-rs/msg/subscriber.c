#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <signal.h>
#include <unistd.h>
#include "dds/dds.h"

// 全局变量用于优雅退出
static volatile sig_atomic_t done = 0;

// 信号处理器
void signal_handler(int sig) {
    (void)sig;
    done = 1;
}

// 打印字节流的辅助函数
void print_bytes(const void* data, size_t size) {
    const unsigned char* bytes = (const unsigned char*)data;
    printf("字节流数据 (%zu bytes):\n", size);
    
    // 按行打印，每行16字节
    for (size_t i = 0; i < size; i += 16) {
        printf("%08zx: ", i);
        
        // 打印十六进制
        for (size_t j = 0; j < 16; j++) {
            if (i + j < size) {
                printf("%02x ", bytes[i + j]);
            } else {
                printf("   ");
            }
            if (j == 7) printf(" ");
        }
        
        printf(" |");
        
        // 打印ASCII字符
        for (size_t j = 0; j < 16 && i + j < size; j++) {
            unsigned char c = bytes[i + j];
            printf("%c", (c >= 32 && c <= 126) ? c : '.');
        }
        
        printf("|\n");
    }
    printf("\n");
}

// 原始数据回调函数
void raw_data_callback(dds_entity_t reader, void* arg) {
    (void)arg;
    
    // 使用原始缓冲区读取
    void* samples[1];
    dds_sample_info_t infos[1];
    samples[0] = NULL;
    
    // 读取原始数据
    int ret = dds_read(reader, samples, infos, 1, 1);
    if (ret < 0) {
        printf("读取数据失败: %s\n", dds_strretcode(-ret));
        return;
    }
    
    if (ret > 0 && infos[0].valid_data) {
        printf("=== 接收到原始数据 ===\n");
        printf("样本状态: %s\n", infos[0].valid_data ? "有效" : "无效");
        printf("实例状态: %d\n", infos[0].instance_state);
        printf("视图状态: %d\n", infos[0].view_state);
        printf("样本状态: %d\n", infos[0].sample_state);
        
        // 打印时间戳
        printf("源时间戳: %lld ns\n", (long long)infos[0].source_timestamp);
        
        // 尝试获取数据大小并打印字节流
        if (samples[0] != NULL) {
            // 由于我们不知道确切的数据大小，我们可以尝试一些启发式方法
            // 或者设置一个合理的最大大小来读取
            size_t max_size = 1024; // 假设最大1KB
            print_bytes(samples[0], max_size);
        }
        
        printf("========================\n\n");
    }
    
    // 释放读取的样本
    dds_return_loan(reader, samples, ret);
}

// 创建一个简单的字节数组类型描述符
static const dds_topic_descriptor_t raw_bytes_desc = {
    .m_size = sizeof(void*),
    .m_align = sizeof(void*),
    .m_flagset = DDS_TOPIC_NO_OPTIMIZE,
    .m_nkeys = 0,
    .m_typename = "RawBytes",
    .m_keys = NULL,
    .m_nops = 0,
    .m_ops = NULL,
    .m_meta = ""
};

int main(int argc, char** argv) {
    dds_entity_t participant;
    dds_entity_t topic;
    dds_entity_t reader;
    dds_listener_t* listener = NULL;
    dds_qos_t* qos = NULL;
    int ret;
    
    const char* topic_name = "RawDataTopic";
    
    // 允许从命令行指定主题名称
    if (argc > 1) {
        topic_name = argv[1];
    }
    
    printf("启动 CycloneDDS 原始字节流订阅者...\n");
    printf("监听主题: %s\n", topic_name);
    
    // 设置信号处理器
    signal(SIGINT, signal_handler);
    signal(SIGTERM, signal_handler);
    
    // 创建参与者
    participant = dds_create_participant(DDS_DOMAIN_DEFAULT, NULL, NULL);
    if (participant < 0) {
        printf("创建参与者失败: %s\n", dds_strretcode(-participant));
        return EXIT_FAILURE;
    }
    printf("参与者创建成功\n");
    
    // 创建主题 - 使用通用字节数组描述符
    topic = dds_create_topic(participant, NULL, topic_name, NULL, NULL);
    if (topic < 0) {
        printf("创建主题失败: %s\n", dds_strretcode(-topic));
        dds_delete(participant);
        return EXIT_FAILURE;
    }
    printf("主题创建成功\n");
    
    // 创建 QoS 设置
    qos = dds_create_qos();
    if (qos) {
        // 设置可靠性
        dds_qset_reliability(qos, DDS_RELIABILITY_BEST_EFFORT, 0);
        // 设置历史记录
        dds_qset_history(qos, DDS_HISTORY_KEEP_LAST, 1);
        // 设置持久性
        dds_qset_durability(qos, DDS_DURABILITY_VOLATILE);
    }
    
    // 创建监听器
    listener = dds_create_listener(NULL);
    if (listener) {
        dds_lset_data_available(listener, raw_data_callback);
    }
    
    // 创建读取器
    reader = dds_create_reader(participant, topic, qos, listener);
    if (reader < 0) {
        printf("创建读取器失败: %s\n", dds_strretcode(-reader));
        if (qos) dds_delete_qos(qos);
        if (listener) dds_delete_listener(listener);
        dds_delete(participant);
        return EXIT_FAILURE;
    }
    printf("读取器创建成功\n");
    
    // 清理 QoS 和监听器
    if (qos) dds_delete_qos(qos);
    if (listener) dds_delete_listener(listener);
    
    printf("等待原始数据... (按 Ctrl+C 退出)\n");
    printf("提示: 这个订阅者会尝试读取任何发布到 '%s' 主题的原始字节数据\n\n", topic_name);
    
    // 主循环 - 也可以使用轮询方式
    while (!done) {
        // 轮询方式读取数据（作为备用方案）
        void* samples[1];
        dds_sample_info_t infos[1];
        samples[0] = NULL;
        
        ret = dds_read(reader, samples, infos, 1, 1);
        if (ret > 0) {
            if (infos[0].valid_data) {
                printf("=== 轮询检测到数据 ===\n");
                if (samples[0] != NULL) {
                    // 尝试读取一些字节
                    print_bytes(samples[0], 256); // 打印前256字节
                }
                printf("========================\n\n");
            }
            dds_return_loan(reader, samples, ret);
        }
        
        // 短暂休眠
        usleep(100000); // 100ms
    }
    
    printf("\n正在关闭订阅者...\n");
    
    // 清理资源
    ret = dds_delete(participant);
    if (ret != DDS_RETCODE_OK) {
        printf("清理参与者失败: %s\n", dds_strretcode(-ret));
        return EXIT_FAILURE;
    }
    
    printf("订阅者已关闭\n");
    return EXIT_SUCCESS;
}

// 额外的实用函数：创建一个简单的字节数组发布者（用于测试）
void create_test_publisher() {
    // 这个函数可以在另一个程序中使用来测试订阅者
    // 这里只是作为参考实现
}
