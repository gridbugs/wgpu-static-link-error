fn main() {
    env_logger::init();
    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        backends: wgpu::BackendBit::DX12,
    });
    println!("{:?}", adapter);
}
