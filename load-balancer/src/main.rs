
use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    Client,
};
use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};
struct LoadBalancer {
    pods: Vec<String>,
    current: AtomicUsize,
}

impl LoadBalancer {
    fn new(pods: Vec<String>) -> Self {
        LoadBalancer {
            pods,
            current: AtomicUsize::new(0),
        }
    }

    fn next_pod(&self) -> Option<&String> {
        let current = self.current.fetch_add(1, Ordering::SeqCst) % self.pods.len();
        self.pods.get(current)
    }
}

async fn handle_connection(mut client: TcpStream, lb: Arc<LoadBalancer>) {
    if let Some(pod_ip) = lb.next_pod() {
        println!("Routing connection to pod: {}", pod_ip);
        match TcpStream::connect(format!("{}:8080", pod_ip)).await {
            Ok(mut server) => {
                let (mut client_read, mut client_write) = client.split();
                let (mut server_read, mut server_write) = server.split();

                let client_to_server = io::copy(&mut client_read, &mut server_write);
                let server_to_client = io::copy(&mut server_read, &mut client_write);

                tokio::select! {
                    _ = client_to_server => {},
                    _ = server_to_client => {},
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to pod {}: {}", pod_ip, e);
            }
        }
    } else {
        eprintln!("No pods available");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Kubernetes client
    let client = Client::try_default().await?;

    // Get the list of pods in the default namespace
    let pods: Api<Pod> = Api::default_namespaced(client);
    let lp = ListParams::default();
    let pod_list = pods.list(&lp).await?;

    // Extract pod IP addresses
    let pod_ips: Vec<String> = pod_list
        .iter()
        .filter_map(|pod| pod.status.as_ref()?.pod_ip.clone())
        .collect();

    // Create the load balancer
    let lb = Arc::new(LoadBalancer::new(pod_ips));

    // Start the TCP listener
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Load balancer listening on port 8080");

    while let Ok((client, _)) = listener.accept().await {
        let lb_clone = lb.clone();
        tokio::spawn(async move {
            handle_connection(client, lb_clone).await;
        });
    }

    Ok(())
}