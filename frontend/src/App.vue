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
			this.loadFacebook(document, 'script', 'facebook-jssdk');
		},

		data: {
			fbCredentials: {},
		},

		methods: {

			facebookLoaded: async function (FB) {
				console.log("facebook: ", FB);
				FB.getLoginStatus(function (response) {
					if (response.status === "connected") {
						this.fbCredentials = response.authResponse;
					}
					console.log("Login response: ", response);
				});
			},

			loadFacebook: function (d, s, id) {
				let js, fjs = d.getElementsByTagName(s)[0];
				if (d.getElementById(id)) {
					return;
				}
				js = d.createElement(s);
				js.id = id;
				js.src = "https://connect.facebook.net/pl_PL/sdk/debug.js";
				fjs.parentNode.insertBefore(js, fjs);
			},
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
