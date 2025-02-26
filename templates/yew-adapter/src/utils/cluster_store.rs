use std::{cell::RefCell, rc::Rc};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use wallet_adapter::Cluster;
use yew::{Reducible, UseReducerHandle, UseStateHandle};

use super::{GlobalAction, GlobalAppState, NotificationInfo};

pub type ClusterStoreState = UseReducerHandle<ClusterStore>;

#[derive(Debug, Clone, PartialEq)]
pub enum ClusterStoreActions {
    Add {
        name: String,
        endpoint: String,
        network: String,
        global_state: GlobalAppState,
        trigger: UseStateHandle<bool>,
    },
    Remove {
        cluster_name: String,
        global_state: GlobalAppState,
        trigger: UseStateHandle<bool>,
    },
    Set {
        name: String,
        trigger: UseStateHandle<bool>,
        global_state: GlobalAppState,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub(crate) struct ClusterStore {
    pub(crate) clusters: RefCell<Vec<AdapterCluster>>,
    pub(crate) active_cluster: RefCell<AdapterCluster>,
}

impl Reducible for ClusterStore {
    type Action = ClusterStoreActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ClusterStoreActions::Add {
                name,
                endpoint,
                network,
                global_state,
                trigger,
            } => {
                let cluster: Cluster = network.as_str().try_into().unwrap_or_default();

                let new_cluster = AdapterCluster::new()
                    .add_name(&name)
                    .add_endpoint(&endpoint)
                    .add_cluster(cluster);

                let cluster_name = new_cluster.name.clone();

                let cluster_exists = self.clusters.borrow().iter().any(|inner_cluster| {
                    inner_cluster.name.as_bytes() == new_cluster.name.as_bytes()
                        || inner_cluster.endpoint.as_bytes() == new_cluster.endpoint.as_bytes()
                });

                if cluster_exists {
                    global_state.dispatch(GlobalAction::Message(NotificationInfo::error(
                        "Cluster exists, make sure endpoint or name are not the same",
                    )));
                } else {
                    self.clusters.borrow_mut().push(new_cluster);
                    global_state.dispatch(GlobalAction::Message(NotificationInfo::new(format!(
                        "`{cluster_name}` cluster added!"
                    ))))
                }
                trigger.set(true);
            }
            ClusterStoreActions::Set {
                name,
                trigger,
                global_state,
            } => {
                let get_cluster = self.get_cluster(&name);
                let cluster_name = name.clone();
                if let Some(cluster_found) = get_cluster {
                    *self.active_cluster.borrow_mut() = cluster_found;
                    global_state.dispatch(GlobalAction::Message(NotificationInfo::new(format!(
                        "`{cluster_name}` cluster added!"
                    ))));
                    trigger.set(true);
                } else {
                    global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!(
                        "`{cluster_name}` cluster not found!"
                    ))))
                }
            }
            ClusterStoreActions::Remove {
                cluster_name,
                global_state,
                trigger,
            } => {
                let index = self.clusters.borrow().iter().position(|current_cluster| {
                    current_cluster.name.as_bytes() == cluster_name.as_bytes()
                });

                if let Some(index) = index {
                    self.clusters.borrow_mut().remove(index);
                    global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!(
                        "`{cluster_name}` removed!"
                    ))))
                } else {
                    global_state.dispatch(GlobalAction::Message(NotificationInfo::error(format!(
                        "`{cluster_name}` cluster not found!"
                    ))))
                }

                trigger.set(true);
            }
        }

        self
    }
}

impl ClusterStore {
    pub fn new() -> Self {
        let clusters = RefCell::new(vec![
            AdapterCluster::devnet(),
            AdapterCluster::localnet(),
            AdapterCluster::testnet(),
            AdapterCluster::mainnet(),
        ]);

        let active_cluster = RefCell::new(AdapterCluster::devnet());

        Self {
            clusters,
            active_cluster,
        }
    }

    pub fn get_clusters(&self) -> Vec<AdapterCluster> {
        (*self.clusters.borrow()).to_vec()
    }

    pub fn get_networks(&self) -> Vec<Cluster> {
        let mut networks = self
            .clusters
            .borrow()
            .iter()
            .map(|adapter_cluster| adapter_cluster.cluster)
            .collect::<Vec<Cluster>>();

        networks.dedup();

        networks
    }

    pub fn active_cluster(&self) -> AdapterCluster {
        self.active_cluster.borrow().clone()
    }

    pub fn get_cluster(&self, name: &str) -> Option<AdapterCluster> {
        self.clusters
            .borrow()
            .iter()
            .find(|cluster| cluster.name == name)
            .cloned()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct AdapterCluster {
    name: String,
    cluster: Cluster,
    endpoint: String,
}

impl AdapterCluster {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_name(mut self, name: &str) -> Self {
        self.name = name.to_string();

        self
    }

    pub fn add_cluster(mut self, cluster: Cluster) -> Self {
        self.cluster = cluster;

        self
    }

    pub fn add_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = endpoint.to_string();

        self
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn cluster(&self) -> Cluster {
        self.cluster
    }
    pub fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    pub fn query_string(&self) -> String {
        if self.name.as_bytes() == self.cluster.to_string().as_bytes()
            && self.cluster != Cluster::LocalNet
        {
            String::new() + "?cluster=" + self.cluster.to_string().as_str()
        } else {
            String::new()
                + "?cluster=custom&customUrl="
                + utf8_percent_encode(self.endpoint.as_str(), NON_ALPHANUMERIC)
                    .to_string()
                    .as_str()
        }
    }

    pub fn devnet() -> Self {
        AdapterCluster {
            name: "devnet".to_string(),
            cluster: Cluster::DevNet,
            endpoint: Cluster::DevNet.endpoint().to_string(),
        }
    }

    pub fn mainnet() -> Self {
        AdapterCluster {
            name: "mainnet".to_string(),
            cluster: Cluster::MainNet,
            endpoint: Cluster::MainNet.endpoint().to_string(),
        }
    }

    pub fn testnet() -> Self {
        AdapterCluster {
            name: "testnet".to_string(),
            cluster: Cluster::TestNet,
            endpoint: Cluster::TestNet.endpoint().to_string(),
        }
    }

    pub fn localnet() -> Self {
        AdapterCluster {
            name: "localnet".to_string(),
            cluster: Cluster::LocalNet,
            endpoint: Cluster::LocalNet.endpoint().to_string(),
        }
    }
}

impl Default for AdapterCluster {
    fn default() -> Self {
        Self::devnet()
    }
}

impl std::fmt::Display for AdapterCluster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.cluster.display())
    }
}
