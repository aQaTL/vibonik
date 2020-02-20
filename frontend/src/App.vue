<template>
	<div id="app">
		<HelloWorld msg="Welcome to Your Vue.js App"/>
	</div>
</template>

<script>
	import HelloWorld from './components/HelloWorld.vue'

	let app = {
		name: 'App',
		components: {
			HelloWorld
		},
		mounted() {
		},
		methods: {
			facebookLoaded: async function (FB) {
				console.log("facebook: ", FB);
				FB.getLoginStatus(function (response) {
					console.log("Login response: ", response);
				});
			}
		}
	};

	window.fbAsyncInit = function () {
		/*eslint no-undef: "off"*/
		FB.init({
			appId: "196521161566506",
			autoLogAppEvents: true,
			xfbml: false,
			version: 'v6.0'
		});
		FB.AppEvents.logPageView();

		app.methods.facebookLoaded(FB);
	};

	(function (d, s, id) {
		var js, fjs = d.getElementsByTagName(s)[0];
		if (d.getElementById(id)) {
			return;
		}
		js = d.createElement(s);
		js.id = id;
		js.src = "https://connect.facebook.net/pl_PL/sdk/debug.js";
		fjs.parentNode.insertBefore(js, fjs);
	}(document, 'script', 'facebook-jssdk'));

	export default app;
</script>

<style>
	#app {
		font-family: Avenir, Helvetica, Arial, sans-serif;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		text-align: center;
		color: #2C3E50;
		margin-top: 60px;
	}
</style>
