// @generated automatically by Diesel CLI.

diesel::table! {
    mfa (otp) {
        id -> Int4,
        otp -> Int4,
    }
}

diesel::table! {
    session (session_token) {
        user_id -> Int4,
        session_token -> Bytea,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        passkey -> Varchar,
        email -> Nullable<Varchar>,
    }
}

diesel::joinable!(mfa -> users (id));
diesel::joinable!(session -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    mfa,
    session,
    users,
);
