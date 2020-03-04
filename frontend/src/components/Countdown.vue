<template>
	<main>

		<div id="welcome">
			<transition name="fade-title">
				<span class="title" v-if="loaded">WIXATRYK</span>
			</transition>
			<transition name="fade-subtitle">
				<span class="subtitle" v-if="loaded">Najlepsza wixa tego miasta</span>
			</transition>
		</div>

		<transition name="fade-countdown">
			<div id="countdown_timer" v-if="loaded">
				<div class="col_cdt">
					<span class="txt1_cdt days">{{ days | doubleDigit }}</span>
					<span class="txt1_sub_cdt">Dni</span>
				</div>

				<div class="col_cdt">
					<span class="txt1_cdt hours">{{ hours | doubleDigit }}</span>
					<span class="txt1_sub_cdt">Godzin</span>
				</div>

				<div class="col_cdt">
					<span class="txt1_cdt minutes">{{ minutes | doubleDigit }}</span>
					<span class="txt1_sub_cdt">Minut</span>
				</div>

				<div class="col_cdt">
					<span class="txt1_cdt seconds">{{ seconds | doubleDigit }}</span>
					<span class="txt1_sub_cdt">Sekund</span>
				</div>
			</div>
		</transition>

	</main>
</template>

<script>


	export default {
		name: "Countdown",
		data() {
			return {
				deadline: new Date(2020, 4, 21, 18, 0, 0, 0),
				days: 0,
				hours: 0,
				minutes: 0,
				seconds: 0,

				loaded: false,
			};
		},

		mounted: function () {
			this.loaded = true;
		},

		created: function () {
			setInterval(this.updateTimer, 1000);
			this.updateTimer();
		},

		methods: {
			updateTimer() {
				let diff = this.deadline - new Date();
				let s = Math.floor(diff / 1000);

				this.seconds = s % 60;

				let m = Math.floor(s / 60);
				this.minutes = m % 60;

				let h = Math.floor(m / 60);
				this.hours = h % 24;
				this.days = Math.floor(h / 24);
			}
		},

		filters: {
			doubleDigit: function (value) {
				return value.toString().padStart(2, "0");
			}
		}
	}
</script>

<style scoped>

	/* Animacje */

	.fade-title-enter-active, .fade-subtitle-enter-active, .fade-countdown-enter-active {
		transition: opacity 1s ease-in-out;
	}

	.fade-title-enter-to, .fade-subtitle-enter-to, .fade-countdown-enter-to {
		opacity: 1;
	}

	.fade-title-enter, .fade-subtitle-enter, .fade-countdown-enter {
		opacity: 0;
	}

	.fade-subtitle-enter-to {
		transition-delay: 0.5s;
	}

	.fade-countdown-enter-to {
		transition-delay: 1s;
	}

	/* Blok Główny */

	#welcome {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		flex-direction: column;
		align-items: center;
		margin-bottom: 50px;
		margin-top: -6em;
		font-family: "Buran-USSR";
	}

	.title {
		font-size: 200px;
		margin: 0 auto;
		color: white;
		text-shadow: 2px 2px 0 #BCBCBC, 4px 4px 0 #000000;
		margin-bottom: -25px;
	}

	.subtitle {
		font-family: "Poppins-Black";
		font-size: 45px;
		line-height: 1;
		color: #FFFFFF;
		text-transform: uppercase;
		text-shadow: 2px 2px 0 #BCBCBC, 4px 4px 0 #000000;
	}

	@media (max-width: 800px) {
		#welcome {
			margin-bottom: 10px;
		}

		.title {
			text-shadow: 1px 1px 0 #BCBCBC, 2px 2px 0 #000000;
			font-size: calc(5.25rem + 3.3vw);
		}

		.subtitle {
			line-height: 2;
			text-shadow: 1px 1px 0 #BCBCBC, 2px 2px 0 #000000;
			font-size: calc(0.9rem + 1vw);
		}
	}

	@media (max-width: 400px) {
		.title {
			font-size: calc(4.5rem + 1.3vw);
		}
	}

	/* Licznik */

	#countdown_timer {
		width: 65vw;
		margin: 0 auto;
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		text-shadow: -1px -1px 0 #000000, 1px -1px 0 #000000, -1px 1px 0 #000000, 1px 1px 0 #000000;
	}

	.col_cdt {
		width: 130px;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		/* align-items: center; */
	}

	.txt1_sub_cdt {
		font-family: "Poppins-Black";
		font-size: 12px;
		color: #FFFFFF;
		line-height: 1;
		text-transform: uppercase;
		margin-left: 10px;
	}

	.txt1_cdt {
		font-family: "Poppins-Black";
		font-size: 120px;
		line-height: 1;
		color: #FFFFFF;
		text-transform: uppercase;
	}


	@media only screen and (max-width: 1000px) {
		#countdown_timer {
			display: block !important;
			text-align: center;
		}

		.col_cdt {
			display: block;
			text-align: left;
		}

		.txt1_sub_cdt {
			font-size: calc(0.5rem + 3.3vw);
		}

		.txt1_cdt {
			font-size: calc(2.5rem + 3.3vw);
		}
	}


</style>