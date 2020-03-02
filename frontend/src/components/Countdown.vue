<template>
	<main>
		<h1>WIXATRYK</h1>
		<h2>NAJLEPSZA WIXA TEGO MIASTA</h2>
		<div>{{ days | doubleDigit }} DNI</div>
		<div>{{ hours | doubleDigit }} GODZIN</div>
		<div>{{ minutes | doubleDigit }} MINUT</div>
		<div>{{ seconds | doubleDigit }} SEKUND</div>
	</main>
</template>

<script>
	export default {
		name: "Countdown",
		data() {
			return {
				days: 0,
				hours: 0,
				minutes: 0,
				seconds: 0,
			};
		},

		mounted: function () {
			let start = new Date(2020, 4, 21, 18, 0, 0, 0);
			setInterval(() => {
				let diff = start - new Date();
				let s = Math.floor(diff / 1000);

				this.seconds = s % 60;

				let m = Math.floor(s / 60);
				this.minutes = m % 60;

				let h = Math.floor(m / 60);
				this.hours = h % 24;
				this.days = Math.floor(h / 24);
			}, 1000);
		},
		filters: {
			doubleDigit: function (value) {
				return value.toString().padStart(2, "0");
			}
		}
	}
</script>

<style scoped>
	main {
		display: flex;
		flex-direction: column;
		align-items: center;

		font-family: "Staatliches", serif;
		font-size: 30pt;
		color: white;
		text-shadow: 2px 2px 0 #bcbcbc, 4px 4px 0 #000000;
	}

	h1 {
		font-size: 400%;
		padding: 0;
	}
</style>