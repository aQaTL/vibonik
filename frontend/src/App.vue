<template>
	<div id="app">
		<header>
			<nav class="menu">

				<div class="underline"></div>

				<!-- Tutaj podpinać kolejne opcje pod menu (typu te ukryte), strona sama przeliczy ilość elementów w menu i dopasuje ich szerokość ( mounted -> adjustMenu(true)) -->

				<router-link to="/">
					<div class="nav-link" @click="ul(0)" v-if="mobile">
						<font-awesome-icon icon="home"/>
					</div>
					<div class="nav-link" @click="ul(0)" v-else>Strona Główna</div>
				</router-link>

				<router-link to="/Info">
					<div class="nav-link" @click="ul(1)" v-if="mobile">
						<font-awesome-icon icon="info"/>
					</div>
					<div class="nav-link" @click="ul(1)" v-else>Informacje</div>
				</router-link>


				<!-- Tutaj testowo robiłem z v-ifem przełączanie ukrytych funkcji po kliknięciu w logowanie -->
				<router-link to="/Music" v-if="test.zalogowano">
					<div class="nav-link" @click="ul(2)" v-if="mobile">
						<font-awesome-icon icon="music"/>
					</div>
					<div class="nav-link" @click="ul(2)" v-else>Piosenki</div>
				</router-link>

				<router-link to="/Ticket" v-if="test.zalogowano">
					<div class="nav-link" @click="ul(3)" v-if="mobile">
						<font-awesome-icon icon="ticket-alt"/>
					</div>
					<div class="nav-link" @click="ul(3)" v-else>Bilet</div>
				</router-link>

				<router-link to="/Profile" v-if="test.zalogowano">
					<div class="nav-link" @click="ul(4)" v-if="mobile">
						<font-awesome-icon icon="id-card"/>
					</div>
					<div class="nav-link" @click="ul(4)" v-else>Profil</div>
				</router-link>

				<router-link to="/Users" v-if="test.zalogowano">
					<div class="nav-link" @click="ul(5)" v-if="mobile">
						<font-awesome-icon icon="user"/>
					</div>
					<div class="nav-link" @click="ul(5)" v-else>Użytkownicy</div>
				</router-link>

				<!-- Pseudologowanie - usunąć -->
				<router-link to="/" v-if="!test.zalogowano">
					<div class="nav-link" @click="ul(0), pseudologowanie(true)" v-if="mobile">
						<font-awesome-icon icon="key"/>
					</div>
					<div class="nav-link" @click="ul(0), pseudologowanie(true)" v-else>Zaloguj</div>
				</router-link>

				<router-link to="/" v-else>
					<div class="nav-link" @click="ul(0), pseudologowanie(false)" v-if="mobile">
						<font-awesome-icon icon="sign-out-alt"/>
					</div>
					<div class="nav-link" @click="ul(0), pseudologowanie(false)" v-else>Wyloguj</div>
				</router-link>
			</nav>

			<div id="menuSeparator"></div>
		</header>

		<router-view></router-view>
	</div>
</template>

<script>

	import {API} from "./main";

	export default {
		name: 'App',
		mounted() {
			this.loadFacebook();
			this.adjustMenu(true); // Dostosowanie menu do ilości opcji
			this.adjustUnderline(window.location.pathname); // Dostosowanie efektu aktualnie wybranej podstrony
			window.addEventListener('resize', this.handleWindowResize); // Wykrywanie zmiany rozmiaru strony
		},

		data() {
			return {
				siteWidth: document.documentElement.clientWidth,
				mobile: false,

				FB: null,
				fbCredentials: {
					accessToken: "",
					expiresIn: "",
					signedRequest: "",
					userID: "",
				},

				// Do pseudologowania - usunąć
				test: {
					zalogowano: false,
				}
			};
		},

		beforeDestroy: function () {
			window.removeEventListener('resize', this.handleWindowResize)
		},

		methods: {
			// Funkcja do pseudologowania - usunąć
			pseudologowanie: function (type) {
				this.test.zalogowano = type;
				setTimeout(() => {
					this.adjustMenu(true)
				}, 0);
			},


			// Funkcja do "animacji" menu
			ul: function (index) {
				let underlines = document.querySelectorAll(".underline");
				for (let i = 0; i < underlines.length; i++) underlines[i].style.transform = 'translate3d(' + index * 100 + '%,0,0)';
			},

			handleWindowResize(event) {
				this.siteWidth = event.currentTarget.innerWidth;
				this.adjustMenu(false);
			},

			// Funkcja do ustawienia poprawnego podświetlenia opcji 
			adjustUnderline: function (subpage) {
				let index = 0;

				// Tutaj należy dopisywać kolejne przypadki dla każdej podstrony oraz ich index który jest przypisany w menu
				switch (subpage) {
					case "/":
						index = 0;
						break;
					case "/Info":
						index = 1;
						break;
					default:
						index = 0;
						break;
				}
				this.ul(index);
			},

			// Funkcja dopasowania menu
			adjustMenu: function (isNecessaryMenuResize) {
				if (isNecessaryMenuResize) {
					let numberOfChildren = document.getElementsByClassName('menu')[0].getElementsByClassName('nav-link').length;
					// console.log("Ilość elementów: " + numberOfChildren);
					for (let i = 0; i < numberOfChildren; i++) document.getElementsByClassName('nav-link')[i].style.width = "calc(100%/" + numberOfChildren + ")";
					document.getElementsByClassName('underline')[0].style.width = "calc(100%/" + numberOfChildren + ")";
				}
				this.siteWidth <= 1000 ? this.mobile = true : this.mobile = false;
			},

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
						let respData = await resp.json();
						switch (respData.authStatus) {
							case "success":
								this.user = respData;
								this.$store.commit('setUser', respData);
								console.log("Login successful: ", respData);
								break;
							case "newUser":
								this.$store.commit('setUser', respData);
								this.user = respData;
								console.log("New user: ", respData);
								await this.$router.push("/profile");
								break;
							case "fail":
								console.log("Failed to auth");
								break;
							default:
								console.log(`unknown authStatus: `, respData);
						}
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


	@font-face {
		font-family: "Staatliches";
		src: url("assets/fonts/Staatliches-Regular.ttf");
	}

	@font-face {
		font-family: "Poppins-Black";
		src: url("assets/fonts/Poppins-Black.ttf");
	}

	@font-face {
		font-family: "Buran-USSR";
		src: url("assets/fonts/Buran-USSR.ttf");
	}

	body {
		//background-color: $base00;
		color: $base05;
		margin: 0;
		padding: 0;

		background-image: url("assets/imgs/background.jpg");
		background-size: cover;
		background-attachment: fixed;
	}

	#app {
		width: 100vw;
		height: 100vh;
		display: grid;
		grid-template-columns: 1fr;
		grid-template-areas: "header" "main";

		font-family: Avenir, Helvetica, Arial, sans-serif;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;

		overflow: auto;
	}

	a {
		color: $base07;
		text-decoration: none;
	}

	// a:hover {
	// 	color: $orange;
	// }

	/* MENU */

	#menuSeparator {
		height: 1px;
		background: #353B48;
		width: 100%;
		border-bottom: 1px solid #353B48;
	}

	nav {
		overflow: hidden;
		position: relative;
		white-space: nowrap;
		background: white;
		padding: .5em 0;
		box-shadow: 0 1em 2em rgba(black, .05);
		font-family: "system-ui", sans-serif;
		width: 100%;
	}

	.underline {
		display: inline-block;
		position: absolute;
		z-index: 0;
		bottom: 0;
		left: 0;
		height: 100%;
		background: orange !important;
		pointer-events: none;
		mix-blend-mode: multiply;
		transition: transform .5s ease-in-out;
	}

	.nav-link {
		display: inline-block;
		z-index: 1;
		padding: 1em 0;
		text-align: center;
		cursor: pointer;
	}

	.menu {
		font-weight: bold;
		background: #111111;
		color: white;
	}

</style>
