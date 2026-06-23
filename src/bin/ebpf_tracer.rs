use libbpf_rs::ObjectBuilder;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Loading eBPF syscall tracer...");
    
    let obj_path = std::env::current_dir()?
        .join("ebpf/syscall_tracer.o");
    
    let mut obj = ObjectBuilder::default()
        .open_file(&obj_path)?
        .load()?;
    
    println!("Attaching to tracepoint...");
    
    let mut links = vec![];
    for prog in obj.progs_mut() {
        println!("Attaching: {:?}", prog.name());
        links.push(prog.attach()?);
    }
    
    println!("Maps available:");
    for map in obj.maps() {
        println!("  - {:?}", map.name());
    }
    
    let events_map = obj.maps_mut()
        .find(|m| m.name().to_string_lossy() == "events")
        .ok_or("events map not found")?;
    
    println!("Reading ringbuf events for 10 seconds...");
    
    let mut event_count = 0;
    let start = std::time::Instant::now();
    
    while start.elapsed() < Duration::from_secs(10) {
        match events_map.ringbuf_read() {
            Ok(Some(data)) => {
                event_count += 1;
                if event_count % 100 == 0 {
                    println!("Event {}: {} bytes", event_count, data.len());
                }
            }
            Ok(None) => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
    
    println!("Total events captured: {}", event_count);
    println!("Done.");
    
    Ok(())
}
