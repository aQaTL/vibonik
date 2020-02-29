<template>
	<div>
		{{ countdown }}
	</div>
</template>

<script>
	export default {
		name: "Counter.vue",
		data() {
			return {
				countdown: new Date(),
			};
		},

		mounted: function () {
			setInterval(() => {
				this.countdown = new Date();
			}, 1000);
		}
	}
</script>

<style scoped>

</style>