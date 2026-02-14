use std::str::FromStr;

use ferrisbox::ChatboxClientInstance;
use ferrisbox::packets::client::tell::TellPacket;
use ferrisbox::packets::server::events::command::CommandEvent;
use uuid::Uuid;
use sqlx::Error;

use crate::database::player::Player;
use crate::models::timecard::Timecard;
use crate::state::{self, POOL};
use crate::models::minecraft_player::MinecraftPlayer;

static BOT_NAME: &str = "Katze Clock";


async fn is_user_registered(uuid: &str) -> Result<bool, sqlx::Error> {
    let pool = POOL.get().expect("Failed to get database pool");
    let result = Player::get_by_uuid(pool, Uuid::parse_str(uuid).unwrap()).await;
    match result {
        Ok(_) => Ok(true),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Ok(false),
            _ => Err(err),
        }
    }
}

pub async fn clock_in(client: &ChatboxClientInstance, command: CommandEvent) {
    match is_user_registered(&command.user.uuid).await {
        Ok(true) => {/* Do nothing. */}
        Ok(false) => {return ;}
        Err(err) =>  {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("There was an error while checking your registration. {}", err).to_string(),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
            return ; // Do not continue.
        }
    }
    let pool = POOL.get().expect("Failed to get database pool");
    let player = Player::get_by_uuid(pool, Uuid::parse_str(&command.user.uuid).unwrap()).await;
    match player {
        Ok(player) => {
            let card = Timecard::get(player).await;
            match card {
                Ok(card) => {
                    let success = card.clock_in().await;
                    match success {
                        Ok(_) => {
                            let packet = TellPacket {
                                user: command.user.name.clone(),
                                text: "<green>You have clocked in.</green>".to_string(),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                        Err(err) => {
                            let packet = TellPacket {
                                user: command.user.name.clone(),
                                text: format!("<red>Failed to clock in: {}</red>", err),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                    }
                }
                Err(err) => {
                    let packet = TellPacket {
                        user: command.user.name.clone(),
                        text: format!("<red>Failed to clock in: {}</red>", err),
                        name: Some(BOT_NAME.to_string()),
                        mode: Some("minimessage".to_string()),
                    };
                    client.tell(packet).await;
                }
            }
        }
        Err(err) => {
            let packet = TellPacket {
                user: command.user.name.clone(),
                text: format!("<red>Failed to clock in: {}</red>", err),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
        }
    }
}

pub async fn clock_out(client: &ChatboxClientInstance, command: CommandEvent) {
    match is_user_registered(&command.user.uuid).await {
        Ok(true) => {/* Do nothing. */}
        Ok(false) => {return ;}
        Err(err) =>  {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("There was an error while checking your registration. {}", err).to_string(),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
            return ; // Do not continue.
        }
    }
    let pool = POOL.get().expect("Failed to get database pool");
    let player = Player::get_by_uuid(pool, Uuid::parse_str(&command.user.uuid).unwrap()).await;
    match player {
        Ok(player) => {
            let card = Timecard::get(player).await;
            match card {
                Ok(card) => {
                    let success =card.clock_out(command.args).await;
                    match success {
                        Ok(_) => {
                            let packet = TellPacket {
                                user: command.user.name.clone(),
                                text: "<green>You have clocked out.</green>".to_string(),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                        Err(err) => {
                            let packet = TellPacket {
                                user: command.user.name.clone(),
                                text: format!("<red>Failed to clock out: {}</red>", err),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                    }
                }
                Err(err) => {
                    let packet = TellPacket {
                        user: command.user.name.clone(),
                        text: format!("<red>Failed to clock out: {}</red>", err),
                        name: Some(BOT_NAME.to_string()),
                        mode: Some("minimessage".to_string()),
                    };
                    client.tell(packet).await;
                }
            }
        }
        Err(err) => {
            let packet = TellPacket {
                user: command.user.name.clone(),
                text: format!("<red>Failed to clock out: {}</red>", err),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
        }
    }
}

pub async fn clock_status(client: &ChatboxClientInstance, command: CommandEvent) {
    match is_user_registered(&command.user.uuid).await {
        Ok(true) => {/* Do nothing. */}
        Ok(false) => {return ;}
        Err(err) =>  {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("<red>There was an error while checking your registration. {}", err).to_string(),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
            return ; // Do not continue.
        }
    }
    let pool = POOL.get().expect("Failed to get database pool");
    let player = Player::get_by_uuid(pool, Uuid::parse_str(&command.user.uuid).unwrap()).await;
    match player {
        Ok(player) => {
            let card = Timecard::get(player).await;
            match card {
                Ok(card) => {
                    let status = if card.is_clocked_in() {
                        "clocked in"
                    } else {
                        "clocked out"
                    };
                    let packet = TellPacket {
                        user: command.user.name.clone(),
                        text: format!("<green>You are currently {}", status),
                        name: Some(BOT_NAME.to_string()),
                        mode: Some("minimessage".to_string()),
                    };
                    client.tell(packet).await;
                }
                Err(err) => {
                    let packet = TellPacket {
                        user: command.user.name.clone(),
                        text: format!("<red>Failed to get clock status: {}", err),
                        name: Some(BOT_NAME.to_string()),
                        mode: Some("minimessage".to_string()),
                    };
                    client.tell(packet).await;
                }
            }
        }
        Err(err) => {
            let packet = TellPacket {
                user: command.user.name.clone(),
                text: format!("<red>Failed to get clock status: {}", err),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
        }
    }
}

pub async fn add_user(client: &ChatboxClientInstance,command: CommandEvent) {
    match is_user_registered(&command.user.uuid).await {
        Ok(true) => {/* Do nothing. */}
        Ok(false) => {return ;}
        Err(err) =>  {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("<red>There was an error while checking your registration. {}</red>", err).to_string(),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
            return ; // Do not continue.
        }
    }
    let user = command.args.get(0);
    let pool = state::get_pool();
    let executor = Player::get_by_uuid(pool, Uuid::from_str(&command.user.uuid).unwrap());
    if !executor.await.unwrap().admin { // unwrap is safe here because we've already checked if the user is registered
        let packet = TellPacket{
            user: command.user.name.clone(),
            text: format!("<red>You do not have permission to add users.</red>"),
            name: Some(BOT_NAME.to_string()),
            mode: Some("minimessage".to_string()),
        };
        client.tell(packet).await;
        return ; // Do not continue.
    }

    match user {
        Some(username) => {
            if let Ok(plr) = Player::get_by_username(pool, username.to_string()).await {
                let packet = TellPacket{
                    user: command.user.name.clone(),
                    text: format!("<red>User {} already exists</red>", plr.get_name()),
                    name: Some(BOT_NAME.to_string()),
                    mode: Some("minimessage".to_string()),
                };
                client.tell(packet).await;
                return ;
            }
            let player = MinecraftPlayer::from_mojang(username).await;
            match player {
                Ok(player) => {
                    let uuid = Uuid::from_str(&player.id);
                    match uuid {
                        Ok(uuid) => {
                            let db_player = Player::create(pool, player.name, uuid).await;
                            match db_player {
                                Ok(player) => {
                                    let packet = TellPacket {
                                        user: command.user.name.clone(),
                                        text: format!("<green>Added user {}</green>", player.get_name()),
                                        name: Some(BOT_NAME.to_string()),
                                        mode: Some("minimessage".to_string()),
                                    };
                                    client.tell(packet).await;
                                }
                                Err(err) => {
                                    let packet = TellPacket {
                                        user: command.user.name.clone(),
                                        text: format!("<red>Failed to add user: {}</red>", err),
                                        name: Some(BOT_NAME.to_string()),
                                        mode: Some("minimessage".to_string()),
                                    };
                                    client.tell(packet).await;
                                }
                            }
                        }
                        Err(err) => {
                            let packet = TellPacket {
                                user: command.user.name.clone(),
                                text: format!("<red>Failed to add user: {}</red>", err),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                    }
                }
                Err(err) => {
                    let packet = TellPacket {
                        user: command.user.name.clone(),
                        text: format!("<red>Failed to add user: {}</red>", err),
                        name: Some(BOT_NAME.to_string()),
                        mode: Some("minimessage".to_string()),
                    };
                    client.tell(packet).await;
                }
            }
        }
        None => {
            let packet = TellPacket {
                user: command.user.name.clone(),
                text: format!("<red>Usage: \\adduser <username>;</red>"),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
        }
    }
}

pub async fn remove_user(client: &ChatboxClientInstance,command: CommandEvent) {
    match is_user_registered(&command.user.uuid).await {
        Ok(true) => {/* Do nothing. */}
        Ok(false) => {return ;}
        Err(err) =>  {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("There was an error while checking your registration. {}", err).to_string(),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
            return ; // Do not continue.
        }
    }
    let user = command.args.get(0);
    let pool = state::get_pool();
    let executor = Player::get_by_uuid(pool, Uuid::from_str(&command.user.uuid).unwrap());
    if !executor.await.unwrap().admin { // unwrap is safe here because we've already checked if the user is registered
        let packet = TellPacket{
            user: command.user.name.clone(),
            text: format!("<red>You do not have permission to add users.</red>"),
            name: Some(BOT_NAME.to_string()),
            mode: Some("minimessage".to_string()),
        };
        client.tell(packet).await;
        return ; // Do not continue.
    }
    match user {
        Some(user) => {
            let player = Player::get_by_username(pool, user.to_string()).await;
            match player {
                Ok(player) => {
                    let name: String;
                    {
                        name = player.get_name().to_string(); // get this before removing the user
                    }
                    match player.remove(pool).await {
                        Ok(_) => {
                            let packet = TellPacket{
                                user: command.user.name.clone(),
                                text: format!("<green>User {} has been removed.</green>", name),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        },
                        Err(err) => {
                            let packet = TellPacket{
                                user: command.user.name.clone(),
                                text: format!("<red>There was an error while removing the user. {}</red>", err),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                    }
                }
                Err(err) => {
                    let packet = TellPacket{
                        user: command.user.name.clone(),
                        text: format!("<red>There was an error while removing the user. {}</red>", err),
                        name: Some(BOT_NAME.to_string()),
                        mode: Some("minimessage".to_string()),
                    };
                    client.tell(packet).await;
                }
            }
        },
        None => {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("<red>Usage: \\adduser <username></red>"),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
        }
    }
}

pub async fn demote_user(client: &ChatboxClientInstance, command: CommandEvent) {
    match is_user_registered(&command.user.uuid).await {
        Ok(true) => {/* Do nothing. */}
        Ok(false) => {return ;}
        Err(err) =>  {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("<red>There was an error while checking your registration. {}</red>", err).to_string(),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
            return ; // Do not continue.
        }
    }
    let user = command.args.get(0);
    let pool = state::get_pool();
    let executor = Player::get_by_uuid(pool, Uuid::from_str(&command.user.uuid).unwrap());
    if !executor.await.unwrap().admin { // unwrap is safe here because we've already checked if the user is registered
        let packet = TellPacket{
            user: command.user.name.clone(),
            text: format!("<red>You do not have permission to add users.</red>"),
            name: Some(BOT_NAME.to_string()),
            mode: Some("minimessage".to_string()),
        };
        client.tell(packet).await;
        return ; // Do not continue.
    }
    match user {
        Some(user) => {
            let player = Player::get_by_username(pool, user.to_string()).await;
            match player {
                Ok(player) => {
                    let pool = POOL.get().expect("Failed to get database pool");
                    let success = player.demote(pool).await;
                    match success {
                        Ok(_) => {
                            let packet = TellPacket{
                                user: command.user.name.clone(),
                                text: format!("<green>User {} demoted.</green>", user),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                        Err(err) => {
                            match err {
                                Error::RowNotFound => {
                                    let packet = TellPacket{
                                        user: command.user.name.clone(),
                                        text: format!("<red>User {} not found.</red>", user),
                                        name: Some(BOT_NAME.to_string()),
                                        mode: Some("minimessage".to_string()),
                                    };
                                    client.tell(packet).await;
                                }
                                _ => {
                                    let packet = TellPacket{
                                        user: command.user.name.clone(),
                                        text: format!("<red>Failed to demote user {}.", user),
                                        name: Some(BOT_NAME.to_string()),
                                        mode: Some("minimessage".to_string()),
                                    };
                                    client.tell(packet).await;
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    match err {
                        Error::RowNotFound => {
                            let packet = TellPacket{
                                user: command.user.name.clone(),
                                text: format!("<red>User {} not found.", user),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                        _ => {
                            let packet = TellPacket{
                                user: command.user.name.clone(),
                                text: format!("<red>Failed to demote user {}.", err),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                    }
                    return ;
                }
            }
        }
        None => {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("<red>You must specify a user to demote.</red>"),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
        }
    }
}
pub async fn promote_user(client: &ChatboxClientInstance, command: CommandEvent) {
    match is_user_registered(&command.user.uuid).await {
        Ok(true) => {/* Do nothing. */}
        Ok(false) => {return ;}
        Err(err) =>  {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("<red>There was an error while checking your registration. {}</red>", err).to_string(),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
            return ; // Do not continue.
        }
    }
    let user = command.args.get(0);
    let pool = state::get_pool();
    let executor = Player::get_by_uuid(pool, Uuid::from_str(&command.user.uuid).unwrap());
    if !executor.await.unwrap().admin { // unwrap is safe here because we've already checked if the user is registered
        let packet = TellPacket{
            user: command.user.name.clone(),
            text: format!("<red>You do not have permission to add users.</red>"),
            name: Some(BOT_NAME.to_string()),
            mode: Some("minimessage".to_string()),
        };
        client.tell(packet).await;
        return ; // Do not continue.
    }
    match user {
        Some(user) => {
            let player = Player::get_by_username(pool, user.to_string()).await;
            match player {
                Ok(player) => {
                    let pool = POOL.get().expect("Failed to get database pool");
                    let name = player.get_name().to_string();
                    let success =player.promote(pool).await;
                    match success {
                        Ok(_) => {
                            let packet = TellPacket{
                                user: command.user.name.clone(),
                                text: format!("<green>User {} has been promoted.</green>", name),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        },
                        Err(err) => {
                            let packet = TellPacket{
                                user: command.user.name.clone(),
                                text: format!("<red>Failed to promote user {}.", err),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                    }
                },
                Err(err) => {
                    match err {
                        sqlx::Error::RowNotFound => {
                            let packet = TellPacket{
                                user: command.user.name.clone(),
                                text: format!("<red>User {} not found.</red>", user),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                        _ => {
                            let packet = TellPacket{
                                user: command.user.name.clone(),
                                text: format!("<red>Failed to promote user {}.</red>", user),
                                name: Some(BOT_NAME.to_string()),
                                mode: Some("minimessage".to_string()),
                            };
                            client.tell(packet).await;
                        }
                    }
                }
            }
        }
        None => {
            let packet = TellPacket{
                user: command.user.name.clone(),
                text: format!("<red>You must specify a user to promote.</red>"),
                name: Some(BOT_NAME.to_string()),
                mode: Some("minimessage".to_string()),
            };
            client.tell(packet).await;
        }
    }
}
