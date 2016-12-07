
extern crate vulkano;
use vulkano::instance::{
    PhysicalDevice,
    Instance,
    layers_list,
    InstanceExtensions,
    ApplicationInfo,
    LayerProperties
};
use std::sync::Arc;

fn build_instance() -> Arc<Instance> {
    Instance::new(None,&InstanceExtensions::none(),None).unwrap()
}


fn device_info(x: &PhysicalDevice) {
    println!("Name: {}", x.name());
    println!("\tType: {:?}", x.ty());
    println!("\tVersion: {:?}", x.api_version());
    println!("\tDriver Version: {:?}", x.driver_version());
    println!("\tIndex: {:?}", x.index());
    println!("\tDevice UUID: {:?}", x.uuid());
    println!("\tPci Vendor ID: {:?}", x.pci_vendor_id());
    println!("\tPci Device ID: {:?}", x.pci_device_id());
    for q in x.queue_families() {
        println!("\tQueue ID: {:?}", q.id());
        println!("\tQueue Count: {:?}", q.queues_count());
        println!("\t\tSupports Graphics: {:?}", q.supports_graphics());
        println!("\t\tSupports Compute: {:?}", q.supports_compute());
        println!("\t\tSupports Transfers: {:?}", q.supports_transfers());
        println!("\t\tSupports Sparse Binding: {:?}", q.supports_sparse_binding());
    }
    for h in x.memory_types().filter(|x|x.is_device_local()) {
        println!("\tHeap ID: {:?}", h.id());
        println!("\t\tSize: {:?}", h.heap().size());
        println!("\t\tIs Host Visiable: {:?}", h.is_host_visible());
    }
    let limits = x.limits();
    println!("\tmax compute shared memory size: {:?}", limits.max_compute_shared_memory_size());
    println!("\tmax compute work group count: {:?}", limits.max_compute_work_group_count());
    println!("\tmax compute work group invocations: {:?}", limits.max_compute_work_group_invocations());
    println!("\tmax compute work group size: {:?}", limits.max_compute_work_group_size());
    println!("\tsparse address space size: {:?}", limits.sparse_address_space_size());

}

fn main() {
    for device in PhysicalDevice::enumerate(&build_instance()) {
        device_info(&device);
    }
}
