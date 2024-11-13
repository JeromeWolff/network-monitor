use pnet::datalink::{self, Channel::Ethernet, DataLinkReceiver, NetworkInterface};
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::Packet;
use serde::Serialize;
use std::env::args;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct NetworkMetricsSnapshot {
    packet_count: usize, // Number of packets captured
    byte_count: usize, // Number of bytes captured
    duration: f64, // Duration of the capture session in seconds
}

struct NetworkMetrics {
    packet_count: usize, // Counter for captured packets
    byte_count: usize, // Counter for captured bytes
    start_time: Instant, // Time the capture session started
}

impl NetworkMetrics {
    fn new() -> Self {
        // Create a new instance of the NetworkMetrics with initial values
        Self {
            packet_count: 0,
            byte_count: 0,
            start_time: Instant::now(),
        }
    }

    fn update(&mut self, packet_size: usize) {
        // Updates packet and byte counters based on the packet size
        self.packet_count += 1;
        self.byte_count += packet_size;
    }

    fn snapshot(&self) -> NetworkMetricsSnapshot {
        // Creates a snapshot of the current state of the NetworkMetrics
        NetworkMetricsSnapshot {
            packet_count: self.packet_count,
            byte_count: self.packet_count,
            duration: self.start_time.elapsed().as_secs_f64(),
        }
    }

    fn print(&self) {
        // Prints the current network metrics to the console
        let elapsed = self.start_time.elapsed();
        println!("Packets: {}, Bytes: {}, Duration: {:.2?}", self.packet_count, self.byte_count, elapsed);
    }

    fn save_to_file(&self, filename: &str) {
        // Saves the metrics snapshot to a JSON file
        let snapshot = self.snapshot();
        let json = serde_json::to_string_pretty(&snapshot).unwrap();
        let mut file = File::create(filename).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

fn capture_packets(interface_name: &str, metrics: Arc<Mutex<NetworkMetrics>>) {
    // Sets up a network connection on the specified interface and starts capturing packets
    let interface_names_match =
        |interface: &NetworkInterface| interface.name == interface_name;
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
        .find(interface_names_match)
        .expect("Failed to find interface");
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_tx, rx)) => (_tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel: {}", e)
    };
    loop_datalink_receive(&mut rx, &metrics);
}

fn loop_datalink_receive(rx: &mut Box<dyn DataLinkReceiver>, metrics: &Arc<Mutex<NetworkMetrics>>) {
    // Continuously reads packets from the datalink channel and updates the metrics
    loop {
        match rx.next() {
            Ok(packet) => {
                // Processes each captured packet and updates the metrics
                let packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&packet);
                let packet_size = packet.packet().len();
                let mut metrics = metrics.lock().unwrap();
                metrics.update(packet_size);
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn handle_packet(packet: &EthernetPacket) {
    // Example packet processing (prints the packet to the console)
    println!("Captured a packet: {:?}", packet);
}

fn monitor_metrics(metrics: Arc<Mutex<NetworkMetrics>>) {
    // Starts a background thread that periodically saves and prints metrics every 5 seconds
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            let metrics = metrics.lock().unwrap();
            metrics.print();
            metrics.save_to_file("metrics.json");
        }
    });
}

fn main() {
    // Main function, initializes metrics and starts packet capture and monitoring
    let metrics = Arc::new(Mutex::new(NetworkMetrics::new()));
    monitor_metrics(Arc::clone(&metrics));
    let interface_name = args().nth(1)
        .expect("Please provide a network interface name");
    capture_packets(&interface_name, Arc::clone(&metrics));
}
