mod integration;
// AdiOS Smart Labeling Plugin
// 
// AI-powered smart labeling and annotation platform with context-aware labeling using Organization Brain.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::info;

/// Main plugin structure for AdiOS Smart Labeling service
pub struct SmartLabelingPlugin {
    /// Plugin metadata and configuration
    info: PluginInfo,
    
    /// Current state of the plugin
    state: RwLock<PluginState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginState {
    /// Currently active labeling projects
    pub active_projects: HashMap<Uuid, LabelingProject>,
    
    /// System metrics and health
    pub system_metrics: SystemMetrics,
    
    /// Plugin configuration
    pub config: PluginConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelingProject {
    pub id: Uuid,
    pub name: String,
    pub task_type: String,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub progress: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Created,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub total_projects: u64,
    pub active_project_count: u32,
    pub samples_labeled: u64,
    pub accuracy_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub confidence_threshold: f64,
    pub enable_active_learning: bool,
    pub max_concurrent_projects: u32,
    pub quality_assurance_enabled: bool,
}

impl Default for PluginState {
    fn default() -> Self {
        Self {
            active_projects: HashMap::new(),
            system_metrics: SystemMetrics {
                total_projects: 0,
                active_project_count: 0,
                samples_labeled: 0,
                accuracy_score: 0.95,
            },
            config: PluginConfig {
                confidence_threshold: 0.9,
                enable_active_learning: true,
                max_concurrent_projects: 10,
                quality_assurance_enabled: true,
            },
        }
    }
}

impl SmartLabelingPlugin {
    pub async fn new() -> Result<Self> {
        let info = PluginInfo {
            id: "adios.smart-labeling".to_string(),
            name: "AdiOS Smart Labeling".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "AI-powered smart labeling and annotation platform".to_string(),
            author: "TridentBiz Team".to_string(),
            category: "enterprise".to_string(),
        };
        
        let state = RwLock::new(PluginState::default());
        
        Ok(Self {
            info,
            state,
        })
    }
    
    pub fn name(&self) -> &str {
        &self.info.name
    }
    
    pub fn version(&self) -> &str {
        &self.info.version
    }
    
    pub fn description(&self) -> &str {
        &self.info.description
    }
    
    pub fn pricing_tiers(&self) -> Vec<PricingTier> {
        vec![
            PricingTier {
                name: "Starter".to_string(),
                price: 100000, // $1,000/month
                features: vec![
                    "Basic AI-assisted labeling".to_string(),
                    "Up to 1,000 samples per month".to_string(),
                    "Standard annotation types".to_string(),
                    "Email support".to_string(),
                    "Basic quality checks".to_string(),
                ],
            },
            PricingTier {
                name: "Professional".to_string(),
                price: 500000, // $5,000/month
                features: vec![
                    "Advanced AI-assisted labeling".to_string(),
                    "Up to 10,000 samples per month".to_string(),
                    "Active learning optimization".to_string(),
                    "Custom annotation schemas".to_string(),
                    "Priority support".to_string(),
                    "Team collaboration features".to_string(),
                ],
            },
            PricingTier {
                name: "Enterprise".to_string(),
                price: 2000000, // $20,000/month
                features: vec![
                    "Context-aware labeling with Organization Brain".to_string(),
                    "Unlimited samples".to_string(),
                    "Custom labeling models".to_string(),
                    "Advanced quality assurance".to_string(),
                    "Dedicated support team".to_string(),
                    "On-premises deployment option".to_string(),
                ],
            },
        ]
    }
    
    pub fn supported_tasks(&self) -> Vec<LabelingTask> {
        vec![
            LabelingTask {
                id: "text_classification".to_string(),
                name: "Text Classification".to_string(),
                description: "Classify text documents into predefined categories".to_string(),
                supported_formats: vec!["txt".to_string(), "json".to_string(), "csv".to_string()],
            },
            LabelingTask {
                id: "named_entity_recognition".to_string(),
                name: "Named Entity Recognition".to_string(),
                description: "Identify and classify entities in text".to_string(),
                supported_formats: vec!["txt".to_string(), "json".to_string()],
            },
            LabelingTask {
                id: "image_classification".to_string(),
                name: "Image Classification".to_string(),
                description: "Classify images into predefined categories".to_string(),
                supported_formats: vec!["jpg".to_string(), "png".to_string(), "bmp".to_string()],
            },
            LabelingTask {
                id: "object_detection".to_string(),
                name: "Object Detection".to_string(),
                description: "Detect and locate objects in images".to_string(),
                supported_formats: vec!["jpg".to_string(), "png".to_string()],
            },
        ]
    }
    
    /// Run the plugin's main loop
    pub async fn run(&self) -> Result<()> {
        info!("Starting AdiOS Smart Labeling Plugin v{}", self.version());
        
        // Display plugin information
        info!("Plugin: {}", self.name());
        info!("Description: {}", self.description());
        
        // Display pricing tiers
        info!("Available pricing tiers:");
        for tier in self.pricing_tiers() {
            info!("  {} - ${:.2}/month", tier.name, tier.price as f32 / 100.0);
            for feature in &tier.features {
                info!("    • {}", feature);
            }
        }
        
        // Display supported tasks
        info!("Supported labeling tasks:");
        for task in self.supported_tasks() {
            info!("  {} - {}", task.name, task.description);
        }
        
        // Start the UI
        info!("Starting smart labeling interface...");
        self.run_ui().await?;
        
        Ok(())
    }
    
    async fn run_ui(&self) -> Result<()> {
        // Simple text-based interface for now
        println!("=== AdiOS Smart Labeling Plugin ===");
        println!("AI-powered smart labeling and annotation platform");
        println!();
        println!("Available commands:");
        println!("  1. Create labeling project");
        println!("  2. List supported tasks");
        println!("  3. Show pricing tiers");
        println!("  4. Exit");
        println!();
        
        // Display supported tasks
        println!("Supported Labeling Tasks:");
        for task in self.supported_tasks() {
            println!("  • {} - {}", task.name, task.description);
        }
        
        println!();
        println!("Pricing Tiers:");
        for tier in self.pricing_tiers() {
            println!("  • {} - ${:.2}/month", tier.name, tier.price as f32 / 100.0);
        }
        
        println!();
        println!("Plugin is ready for intelligent data labeling!");
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingTier {
    pub name: String,
    pub price: u32, // in cents
    pub features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelingTask {
    pub id: String,
    pub name: String,
    pub description: String,
    pub supported_formats: Vec<String>,
}

// Entry point for the plugin
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .init();
    
    // Create and run plugin
    let plugin = SmartLabelingPlugin::new().await?;
    plugin.run().await?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_plugin_initialization() {
        let plugin = SmartLabelingPlugin::new().await.unwrap();
        
        // Test basic functionality
        assert_eq!(plugin.name(), "AdiOS Smart Labeling");
        assert_eq!(plugin.version(), env!("CARGO_PKG_VERSION"));
        assert!(!plugin.description().is_empty());
    }

    #[tokio::test]
    async fn test_pricing_tiers() {
        let plugin = SmartLabelingPlugin::new().await.unwrap();
        
        let tiers = plugin.pricing_tiers();
        assert_eq!(tiers.len(), 3);
        
        // Starter tier
        assert_eq!(tiers[0].name, "Starter");
        assert_eq!(tiers[0].price, 100000); // $1,000
        
        // Professional tier
        assert_eq!(tiers[1].name, "Professional");
        assert_eq!(tiers[1].price, 500000); // $5,000
        
        // Enterprise tier
        assert_eq!(tiers[2].name, "Enterprise");
        assert_eq!(tiers[2].price, 2000000); // $20,000
    }

    #[tokio::test]
    async fn test_supported_tasks() {
        let plugin = SmartLabelingPlugin::new().await.unwrap();
        
        let tasks = plugin.supported_tasks();
        assert!(!tasks.is_empty());
        
        // Check for expected tasks
        let task_ids: Vec<String> = tasks.iter().map(|t| t.id.clone()).collect();
        assert!(task_ids.contains(&"text_classification".to_string()));
        assert!(task_ids.contains(&"named_entity_recognition".to_string()));
        assert!(task_ids.contains(&"image_classification".to_string()));
        assert!(task_ids.contains(&"object_detection".to_string()));
    }
}
