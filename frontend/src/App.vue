<template>
	<div id="app">
		<div id="top-bar">
			<div
					ref="fbLoginButton"
					class="fb-login-button"
					data-width=""
					data-size="large"
					data-button-type="login_with"
					data-auto-logout-link="true"
					data-use-continue-as="false"
					data-onlogin="document.facebookLoginCallback()"
					data-onlogout="console.log('logging OUT!')"
			>
			</div>
		</div>
		<div id="navigation-bar">
			<div class="nav-link">
				<router-link to="/signup">Rejestracja</router-link>
			</div>
			<div class="nav-link">
				<router-link to="/">Główna</router-link>
			</div>
			<div class="nav-link">
				<router-link to="/ticket">Bilet</router-link>
			</div>
			<div class="nav-link">
				<router-link to="/Users">Użytkownicy</router-link>
			</div>
		</div>

		<router-view></router-view>
	</div>
</template>

<script>
	import {API} from "./main";

	export default {
		name: 'App',
		mounted() {
			this.loadFacebook();
		},

		data() {
			return {
				FB: null,
				fbCredentials: {
					accessToken: "",
					expiresIn: "",
					signedRequest: "",
					userID: "",
				},
			};
		},

		methods: {
			loadFacebook: function () {
				let js, fjs = document.getElementsByTagName("script")[0];
				if (document.getElementById("facebook-jssdk") || window.fbAsyncInit !== undefined) {
					return;
				}
				window.fbAsyncInit = () => {
					/*eslint no-undef: "off"*/
					FB.init({
						appId: "196521161566506",
						autoLogAppEvents: true,
						xfbml: true,
						version: 'v6.0'
					});
					FB.AppEvents.logPageView();

					this.FB = FB;
					this.facebookLoaded();
				};

				document.facebookLoginCallback = () => {
					this.facebookLogIn()
				};

				js = document.createElement("script");
				js.id = "facebook-jssdk";
				js.crossOrigin = "anonymous";
				js.src = "https://connect.facebook.net/pl_PL/sdk/debug.js";
				fjs.parentNode.insertBefore(js, fjs);
			},

			facebookLogIn: async function () {
				console.log("facebook: ", this.FB);

				this.FB.getLoginStatus((response) => {
					console.log("calling getLoginStatus");
					if (response.status === "connected") {
						this.fbCredentials = response.authResponse;
						this.facebookLoginSuccessful();
					}
					console.log("Login response: ", response);
				});
			},

			facebookLoginSuccessful: async function () {
				this.FB.api("/me", (response) => {
					console.log("USER INFO: ", response);
				});

				let resp = await fetch(API + "auth", {
					method: "POST",
					headers: {
						"Content-Type": "application/json"
					},
					body: JSON.stringify(this.fbCredentials),
				});
				switch (resp.status) {
					case 200: {
						let authStatus = await resp.json();
						switch (authStatus) {
							default:
								console.log(`unknown authStatus: ${authStatus}`);
						}
						console.log(`AuthStatus: ${authStatus}`);
						break;
					}
					case 201:
					case 500:
				}
			},

			facebookLoaded: async function () {
				this.facebookLogIn();
			},
		}
	}
</script>

<style lang="scss">
	$base00: hsl(50, 5%, 12%); /* #20201d */
	$base01: hsl(50, 6%, 15%); /* #292824 */
	$base02: hsl(50, 8%, 40%); /* #6e6b5e */
	$base03: hsl(50, 9%, 45%); /* #7d7a68 */
	$base04: hsl(50, 11%, 55%); /* #999580 */
	$base05: hsl(50, 13%, 60%); /* #a6a28c */
	$base06: hsl(50, 35%, 86%); /* #e8e4cf */
	$base07: hsl(50, 87%, 96%); /* #fefbec */
	$base08: hsl(0, 67%, 53%); /* #d73737 */
	$base09: hsl(25, 83%, 39%); /* #b65611 */
	$base0a: hsl(50, 80%, 38%); /* #ae9513 */
	$base0b: hsl(100, 50%, 45%); /* #60ac39 */
	$base0c: hsl(162, 70%, 40%); /* #1fad83 */
	$base0d: hsl(225, 67%, 64%); /* #6684e1 */
	$base0e: hsl(287, 60%, 58%); /* #b854d4 */
	$base0f: hsl(349, 65%, 52%); /* #d43552 */

	$red: $base08;
	$orange: $base09;
	$yellow: $base0a;
	$green: $base0b;
	$cyan: $base0c;
	$blue: $base0d;
	$violet: $base0e;
	$magenta: $base0f;

	body {
		background-color: $base00;
		color: $base05;
		margin: 0;
	}

	a {
		color: $base0a;
		text-decoration: none;
	}

	#app {
		display: flex;
		flex-direction: column;
		align-items: center;

		font-family: Avenir, Helvetica, Arial, sans-serif;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
	}

	#top-bar {
		align-self: stretch;
		text-align: center;
		border-bottom: 1px solid $base00;
		padding: 0.5em;
		background-color: $base01;
	}

	#navigation-bar {
		position: fixed;
		bottom: 0;
		left: 0;
		width: 100vw;

		display: flex;

		justify-content: center;
		background-color: $base01;
	}

	#navigation-bar > .nav-link {
		padding: 1em;
		border: 1px solid black;
	}

	.pulse {
		box-shadow: 0 0 0 rgba(204, 169, 44, 0.4);
		animation: pulse 2s infinite;
	}

	.pulse:hover {
		animation: none;
	}

	@keyframes pulse {
		0% {
			-moz-box-shadow: 0 0 0 0 rgba(204, 169, 44, 0.4);
			box-shadow: 0 0 0 0 rgba(204, 169, 44, 0.4);
		}
		70% {
			-moz-box-shadow: 0 0 0 10px rgba(204, 169, 44, 0);
			box-shadow: 0 0 0 10px rgba(204, 169, 44, 0);
		}
		100% {
			-moz-box-shadow: 0 0 0 0 rgba(204, 169, 44, 0);
			box-shadow: 0 0 0 0 rgba(204, 169, 44, 0);
		}
	}
</style>
