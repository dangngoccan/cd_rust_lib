pub mod cdk_serials;
pub mod cdk_os_path;
pub mod cdk_hexencode;
pub mod cdk_base64;
pub mod cdk_command;

fn main() {

    cdk_serials::print_port_list();
    cdk_os_path::print_current_path();

}