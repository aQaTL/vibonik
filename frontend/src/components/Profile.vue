<template>
	<div>
		<div class="user-form">
			<div class="paid"><h3>Zapłacono: {{ user.paid }}</h3>
			</div>
			<label for="">
				Preferencje żywieniowe
			</label>
			<div class="item-select">
				<div
						class="item"
						v-bind:class="{ 'item-selected': item.value === user.foodPreferences.value }"
						v-for="item in foodPreferences"
						v-bind:key="item.value"
						@click="user.foodPreferences = item">{{ item.text }}
				</div>
			</div>
			<input type="button" value="Zapisz" @click="save">

		</div>
	</div>
</template>

<script>
	import {API} from "../main";

	export const foodPreferences = [
		{text: "Standard", value: "none"},
		{text: "Wegetariańskie", value: "Vege"},
		{text: "Wegańskie", value: "Vega"},
	];
	export default {
		name: "Profile",

		data() {
			return {
				user: {
					foodPreferences: foodPreferences[0],
					birthday: null,
					gender: null,
					paid: 0,
				},
				foodPreferences: foodPreferences,
			};
		},

		methods: {
			save: async function () {
				console.log("state: ", this.$store.state.user);
				let resp = await fetch(API + "update_user", {
					method: "POST",
					headers: {
						"Content-Type": "application/json"
					},
					body: JSON.stringify({
						userID: this.$store.state.user.userID,
						uuid: this.$store.state.user.uuid,
						foodPreferences: this.user.foodPreferences.value,
					}),
				});
				resp = await resp.json();
				switch (resp.status) {
					default:
						console.log("unknown response status", resp)
				}
			}
		}
	}
</script>

<style scoped lang="scss">
	.user-form {
		display: grid;
		grid-gap: 0.5em;
		grid-auto-columns: minmax(400px, 1fr);
	}

	.item-select {
		border: 1px #6E6B5E solid;
		border-radius: 4px;
	}

	.item {
		padding: 0.5em;
		text-align: center;
	}

	.item-selected, .item:active {
		background: hsl(349, 65%, 52%);
		color: hsl(50, 35%, 86%);
	}

	.item:hover {
		cursor: pointer;
	}

	.paid {
		padding: 0.5em;
		color: hsl(162, 70%, 40%);
		justify-self: center;
	}

	h3 {
		margin: 0;
	}

	input[type=button] {
		color: hsl(50, 13%, 60%);
		background: hsl(50, 6%, 15%);
		border: 1px solid hsl(100, 50%, 45%);
		border-radius: 4px;
		padding: 0.5em;
		cursor: pointer;
	}
</style>