use sys_info;

fn main() {

    println!("===============SYSTEM INFORMATION====================\n\n\n");
    println!("hostname: {}\n", sys_info::hostname().unwrap());
    // println!("Boottime: {:?}", sys_info::boottime().unwrap());
    println!("CPU num: {}\n", sys_info::cpu_num().unwrap());
    println!("CPU speed: {}\n", sys_info::cpu_speed().unwrap());
    println!("Disk info: {:?}\n", sys_info::disk_info().unwrap());
    println!("OS release: {}\n", sys_info::os_release().unwrap());
    println!("OS type: {}\n", sys_info::os_type().unwrap());
    println!("PROC total: {}\n", sys_info::proc_total().unwrap());
    println!("\n==============by: github - andrrff===================");
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
}
