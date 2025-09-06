cmake_minimum_required(VERSION 3.16)
project(raw_subscriber LANGUAGES C)

if(NOT TARGET CycloneDDS::ddsc)
  # Find the CycloneDDS package.
  find_package(CycloneDDS REQUIRED)
endif()

# 创建原始字节流订阅者（不依赖IDL）
add_executable(RawSubscriber subscriber.c)
target_link_libraries(RawSubscriber CycloneDDS::ddsc)