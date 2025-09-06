mod bindings;
use std::ffi::CString;
use std::ptr;

use crate::bindings::*;

fn main() {

        // let mut status: u32 = 0;

        // 创建 Participant
        let participant: dds_entity_t = unsafe { dds_create_participant(UINT32_MAX, ptr::null(), ptr::null()) };
        if participant < 0 {
            let err_msg = unsafe { dds_strretcode(-participant) };
            let err_cstr = unsafe { std::ffi::CStr::from_ptr(err_msg) };
            println!("dds_create_participant: {}", err_cstr.to_string_lossy());
        }
        //  let des =dds_topic_descriptor::default();
        // 创建 Topic
        let topic_name = CString::new("rt/HelloWorldData_Msg").unwrap();
        let topic: dds_entity_t = unsafe { dds_create_topic(
            participant,
            &HelloWorldData_Msg_desc,
            topic_name.as_ptr(),
            ptr::null(),
            ptr::null(),
        ) };
        if topic < 0 {
            let err_msg = unsafe { dds_strretcode(-topic) };
            let err_cstr = unsafe { std::ffi::CStr::from_ptr(err_msg) };
			println!("dds_create_topic: {}", err_cstr.to_string_lossy());
        }

		let publisher = unsafe { dds_create_publisher(participant, ptr::null(), ptr::null()) };
		
        // 创建 Writer
        let writer: dds_entity_t = unsafe { dds_create_writer(publisher, topic, ptr::null(), ptr::null()) };
        if writer < 0 {
            let err_msg = unsafe { dds_strretcode(-writer) };
            let err_cstr = unsafe { std::ffi::CStr::from_ptr(err_msg) };
			println!("dds_create_writer: {}", err_cstr.to_string_lossy());
        }

    let rc = unsafe { dds_set_status_mask(writer,DDS_LC_TOPIC) };
    let mut msg = HelloWorldData_Msg::default();
loop {	
    msg.userID += 1;
    let message_cstring = CString::new("Hello World").unwrap();
    msg.message = message_cstring.as_ptr() as *mut i8;
    let message_str = unsafe { std::ffi::CStr::from_ptr(msg.message) }.to_string_lossy();
    println!("Message (%{}, {})\n", msg.userID, message_str);
    unsafe { dds_write(writer, &msg as *const _ as *const std::ffi::c_void) };
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

