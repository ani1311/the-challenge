struct Friendship {
    id: String,
    requester_id: String,
    receiver_id: String,
    status: FriendshipStatus,
}

enum FriendshipStatus {
    Pending,
    Accepted,
    Rejected,
}
