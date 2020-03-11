table! {
	users (id) {
		id -> Int4,
		fb_id -> Text,
		access_token -> Nullable<Text>,
		uuid -> Uuid,
		login -> Nullable<Text>,
		password_hash -> Nullable<Text>,
		role -> Nullable<Text>,
		name -> Text,
		pesel -> Nullable<Int8>,
		email -> Nullable<Text>,
		birthday -> Nullable<Date>,
		gender -> Nullable<Bpchar>,
		food_preferences -> Nullable<Text>,
		paid -> Int8,
	}
}
