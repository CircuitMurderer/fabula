use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientMessage {
    pub request_id: u64,
    pub command: ClientCommand,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Response {
        request_id: u64,
        result: ServerResponse,
    },
    Push(ServerPush),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientCommand {
    Register {
        username: String,
        password: String,
    },
    Login {
        username: String,
        password: String,
    },
    Logout,
    CreateRoom {
        room_name: String,
    },

    JoinRoom {
        room_id: String,
    },
    LeaveRoom {
        room_id: String,
    },
    SendRoomMessage {
        room_id: String,
        text: String,
    },
    ListRooms,
    ListMyRooms,
    Ping,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerResponse {
    Ok,
    Error {
        code: ErrorCode,
        message: String,
    },
    LoginOk {
        user_id: String,
    },
    RegisterOk {
        user_id: String,
    },
    RoomCreated {
        room_id: String,
        room_name: String,
    },
    RoomsList {
        rooms: Vec<RoomInfo>,
    },
    MyRoomsList {
        rooms: Vec<RoomInfo>,
    },
    Pong,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerPush {
    RoomMessage {
        room_id: String,
        from_user: String,
        text: String,
        timestamp_ms: i64,
    },
    RoomUserJoined {
        room_id: String,
        user_id: String,
    },
    RoomUserLeft {
        room_id: String,
        user_id: String,
    },
    SystemMessage {
        text: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoomInfo {
    pub room_id: String,
    pub room_name: String,
    pub member_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum ErrorCode {
    Unknown,
    BadRequest,
    NotAuthenticated,
    InvalidCredentials,
    UserAlreadyExists,
    RoomNotFound,
    NoRoomPermission,
    InternalError,
}
