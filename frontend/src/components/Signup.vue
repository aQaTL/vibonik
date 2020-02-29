<template>
	<div>
		<div class="user-form">
			<label for="">
				Preferencje żywieniowe
			</label>
			<select v-model="user.food_preferences" :size="food_preferences.length">
				<option
						v-for="preference in food_preferences"
						v-bind:key="preference.value"
						v-bind:value="preference">
					{{ preference.text }}
				</option>
			</select>
			<input type="text">
			<input type="button" value="ok">

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

	}
</script>

<style scoped>
	.user-form {
		display: grid;
	}
	select {
		overflow: auto;
	}
</style>