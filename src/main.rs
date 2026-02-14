use crate::{chatbox::chatbox_commands::{clock_in, clock_out, clock_status, add_user, remove_user,promote_user, demote_user}, database::player::Player};
use ferrisbox::{
    ChatboxClientInstance,
    packets::server::{PacketType, events::EventType},
};
use futures::{StreamExt};
use sqlx::postgres::PgPool;
use std::{env, sync::Arc};
use uuid::Uuid;
pub mod chatbox;
pub mod database;
pub mod models;
pub mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();
    let license = env::var("FERRISBOX_LICENSE").expect("FERRISBOX_LICENSE not set in .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let webhook_url = env::var("WEBHOOK_URL").expect("WEBHOOK_URL is not set in .env file");
    state::init_webhook(webhook_url);
    let initial_admin_username = env::var("INITIAL_ADMIN_USERNAME");
    let initial_admin_uuid = env::var("INITIAL_ADMIN_UUID");
    //
    let pool = PgPool::connect(&database_url).await?;
    tracing::info!("Running migrations");
    sqlx::migrate!().run(&pool).await?;
    tracing::info!("Migrations completed");
    state::init_pool(pool);
    let pool = state::get_pool();

    let admins = Player::get_admins(pool).await;
    // Initialize admin user if none present in database
    if let Ok(players) = admins {
        tracing::info!("Got list");
        if players.is_empty() {
            tracing::info!("No admin users found");
            if let (Ok(username), Ok(uuid)) = (initial_admin_username, initial_admin_uuid) {
                tracing::info!("Creating admin user");
                let uuid = Uuid::parse_str(&uuid).expect("Invalid UUID");
                let admin = Player::create(pool, username, uuid).await?;
                admin.promote(pool).await?;
            }
            else {
                // Handle error case when username or uuid is not provided
                return Err("INITIAL_ADMIN_USERNAME and INITIAL_ADMIN_UUID not set in .env file, with no existing admin user".into());
            }
        }
    }
    else {
        tracing::error!("Failed to get admin users: {}",admins.unwrap_err());
    }

    let (client, mut events) = ChatboxClientInstance::new(license, None).await;
    let client = Arc::new(client);

    tokio::spawn(async move {
        while let Some(server_packet) = events.next().await {
            match server_packet.packet_type {
                PacketType::Event(event) => match event.event {
                    EventType::Command(command) => match command.command.as_str() {
                        "clockin" => {
                            let client = client.clone();
                            tokio::spawn(async move {
                                clock_in(&client, command).await;
                            });
                        }
                        "clockout" => {
                            let client = client.clone();
                            tokio::spawn(async move {
                                clock_out(&client, command).await;
                            });
                        }
                        "clockstatus" => {
                            let client = client.clone();
                            tokio::spawn(async move {
                                clock_status(&client, command).await;
                            });
                        }
                        "adduser" => {
                            let client = client.clone();
                            tokio::spawn(async move {
                                add_user(&client, command).await;
                            });
                        }
                        "removeuser" => {
                            let client = client.clone();
                            tokio::spawn(async move {
                                remove_user(&client, command).await;
                            });
                        }
                        "promoteuser" => {
                            let client = client.clone();
                            tokio::spawn(async move {
                                promote_user(&client, command).await;
                            });
                        }
                        "demoteuser" => {
                            let client = client.clone();
                            tokio::spawn(async move {
                                demote_user(&client, command).await;
                            });
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    tracing::error!("Something disconnected");
    });
    tokio::signal::ctrl_c().await?;

    Ok(())
}
