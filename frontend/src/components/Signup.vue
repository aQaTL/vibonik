<template>
	<div>
		<div class="user-form">
			<label for="">
				Preferencje żywieniowe
			</label>
			<div class="item-select">
				<div
						class="item"
						v-bind:class="{ 'item-selected': item.value === user.food_preferences.value }"
						v-for="item in food_preferences"
						v-bind:key="item.value"
						@click="user.food_preferences = item">{{ item.text }}
				</div>
			</div>
			<input type="button" value="Zapisz" @click="save">

		</div>
	</div>
</template>

<script>
	export const food_preferences = [
		{text: "Brak", value: "none"},
		{text: "Vege", value: "Vege"},
	];
	export default {
		name: "Signup",
		data() {
			return {
				user: {
					userID: "",
					name: "",
					food_preferences: food_preferences[0],
					birthday: null,
					gender: null,
					paid: 0,
				},
				food_preferences,
			};
		},

		methods: {
			save: async function () {

			}
		}
	}
</script>

<style scoped lang="scss">
	.user-form {
		display: grid;
	}

	.item-select {
		border: 1px #6e6b5e solid;
	}

	.item {
		padding: 0.5em;
	}

	.item-selected {
		background: hsl(349, 65%, 52%);
		color: hsl(50, 35%, 86%);
	}

	.item:hover {
		cursor: pointer;
	}
</style>