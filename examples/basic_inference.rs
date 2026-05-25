use hailo_rs::hw::HailoVDevice;
use hailo_rs::infer::{HailoHef, HailoNetworkGroup, HailoVStreams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Create a virtual device — HailoRT auto-binds to the physical chip.
    let vdevice = HailoVDevice::new()?;
    println!("Connected to Hailo device.");

    // Step 2: Load a Hailo Execution Format (HEF) model.
    let hef = HailoHef::from_file("model.hef")?;

    // Step 3: Configure the model onto the virtual device (via PCIe).
    let network = HailoNetworkGroup::configure(&vdevice, &hef)?;

    // Step 4: Open I/O virtual streams.
    let vstreams = HailoVStreams::create(&network)?;

    // Step 5: Write input tensor and read output tensor.
    //
    // Replace INPUT_SIZE / OUTPUT_SIZE with the actual byte counts for your model.
    // const INPUT_SIZE: usize = 960 * 544 * 3;
    // const OUTPUT_SIZE: usize = 120 * 68;
    //
    // let input: Vec<u8> = vec![128u8; INPUT_SIZE];
    // vstreams.write_input(input.as_ptr(), input.len())?;
    //
    // let mut output: Vec<u8> = vec![0u8; OUTPUT_SIZE];
    // vstreams.read_output(output.as_mut_ptr(), output.len())?;
    //
    // println!("Inference done. Output bytes: {}", output.len());

    drop(vstreams);
    drop(network);
    // vdevice released on drop — HailoRT cleans up the firmware session.

    Ok(())
}
