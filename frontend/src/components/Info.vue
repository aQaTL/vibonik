<template>
	<main>
		<meta rel="stylesheet" href="https://unpkg.com/vue-my-photos/dist/lightbox.css">
		<section id="information_box">
			<div class="menuContainer">
				<div class="menuContainer_title">
					Spis treści
				</div>
				<div class="menuContainer_list">
					<ol class="menuContainer_options">
						<li><a href="#informacje">Informacje Ogólne</a></li>
						<li><a href="#regulamin">Regulamin</a></li>
						<li><a href="#lokalizacja">Lokalizacja</a></li>
						<li><a href="#galeria">Galeria</a></li>
					</ol>
				</div>
			</div>

			<div class="contentContainer">

				<article><a id="informacje"></a>
					<h2 class="heading">
						<div class="heading_text">Informacje Ogólne</div>
					</h2>
					Etiam tristique orci a ligula vestibulum placerat. Aliquam vitae lorem interdum, imperdiet
					purus id, consectetur ipsum. Duis et elit neque. Duis euismod, leo sit amet volutpat
					pellentesque, turpis turpis rutrum dui, in commodo magna magna eu justo. Suspendisse
					ornare dui vitae urna lobortis, et dignissim nisi porttitor. Aliquam mattis pharetra
					tellus vestibulum tincidunt. Ut eget imperdiet augue.
					<br><br>
					In mattis eget orci et suscipit. Pellentesque dapibus, urna ut dictum tincidunt, metus
					eros condimentum urna, sit amet aliquam diam turpis at enim. Aenean auctor dignissim
					efficitur. Donec facilisis urna non finibus vehicula. Cras auctor eget sem nec aliquam.
					Sed cursus commodo tellus, vel consequat mauris auctor in. Sed dui neque, tincidunt
					finibus pellentesque quis, pellentesque a velit.
				</article>

				<article><a id="regulamin"></a>
					<h2 class="heading">
						<div class="heading_text">Regulamin</div>
					</h2>
					Coś tutaj będzie
				</article>

				<article style="text-align:center;"><a id="lokalizacja"></a>
					<h2 class="heading">
						<div class="heading_text">Lokalizacja</div>
					</h2>
					<iframe class="localization"
									src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2582.6872930508157!2d21.77954250978024!3d49.66019104553675!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x473c46253abe1567%3A0xc8612fafb0d56ccb!2sKompleks%20Rekreacyjno%20-%20Rozrywkowy%20DWA%20-%20SERCA!5e0!3m2!1spl!2spl!4v1583330882011!5m2!1spl!2spl"
									allowfullscreen=""></iframe>
					<div class="caption">Miejsce imprezy - "Dwa Serca"</div>
				</article>

				<article><a id="galeria"></a>
					<h2 class="heading">
						<div class="heading_text">Galeria</div>
					</h2>
					<div id="gallery">
						<transition-group name="thumbnailfade" tag="div">
							<img
									v-for="thumb in filteredImages" :key="thumb.id"
									@click="showLightbox(thumb.name)" :src="thumb.name" :alt="thumb.alt"
									:title="thumb.alt"/>
						</transition-group>

						<lightbox ref="lightbox"
											:images="images"
											:filter="galleryFilter"
											:timeoutDuration="3000"
						></lightbox>
					</div>
				</article>

			</div>
		</section>
	</main>
</template>

<script>

	var imageList = [{
		'name': require("../assets/imgs/outside.jpg"),
		'alt': 'Budynek z zewnątrz',
		'filter': 'nature',
		'id': 'image1'
	},
		{
			'name': require("../assets/imgs/inside.jpg"),
			'alt': 'Środek sali',
			'filter': 'animals',
			'id': 'image2'
		},
		{
			'name': require("../assets/imgs/corridor.jpg"),
			'alt': 'Korytarz',
			'filter': 'nature',
			'id': 'image3'
		},
		{
			'name': require("../assets/imgs/desserts.jpg"),
			'alt': 'Przykładowe desery',
			'filter': 'nature',
			'id': 'image3'
		},
		{
			'name': require("../assets/imgs/beverages.jpg"),
			'alt': 'Stół z napojami',
			'filter': 'nature',
			'id': 'image3'
		}];

	export default {
		name: 'Info',
		data() {
			return {
				images: imageList,
				galleryFilter: 'all'
			}
		},
		methods: {
			showLightbox: function (imageName) {
				this.$refs.lightbox.show(imageName);
			},
			updateFilter(filterName) {
				this.galleryFilter = filterName;
			}
		},
		computed: {
			filteredImages: function () {
				if (this.galleryFilter === 'all') return this.images;
				else return this.images.filter(image => image.filter === this.galleryFilter);
			}
		}
	}
</script>

<style scoped>

	/* .heading:before {
		content: " ";
		border-top: solid 1px orange;
		position: absolute;
		bottom: 50%;
		left: 0;
		width:calc((80vw - 357px)/2);
	}

	.heading:after {
		content: " ";
		border-top: solid 1px orange;
		position: absolute;
		bottom: 50%;
		right: 0;
		width:calc((80vw - 357px)/2);
	} */

	/* LIGHTBOX */
	#gallery {
		width: 950px;
		margin: 0 auto;
		text-align: center;
	}

	@media (max-width: 1150px) {
		#gallery {
			width: 100%;
		}
	}

	img {
		width: 270px;
		height: 180px;
		margin: 20px;
		border-radius: 3px;
		cursor: pointer;
		transition: all 0.4s ease;
	}

	.thumbnailfade-leave-active,
	.thumbnailfade-enter-active {
		transition: all 0.4s ease;
	}

	.thumbnailfade-enter-active {
		transition-delay: 0.4s;
	}

	.thumbnailfade-enter,
	.thumbnailfade-leave-to {
		opacity: 0;
	}

	.heading {
		position: relative;
		text-align: center;
		color: black;
	}

	.heading_text {
		position: relative;
		display: inline-block;
		padding: .2em 2em;
		max-width: 80%;
		font-size: 28px;

	}

	.heading_text:before { /* lewa ramka */
		content: " ";
		border: solid 4px white;
		border-right: 0px;
		position: absolute;
		left: 1rem;
		top: 0;
		bottom: 0;
		width: .4em;
		right: 0;
	}

	.heading_text:after { /* prawa ramka */
		content: "";
		border: solid 4px white;
		border-left: 0px;
		position: absolute;
		right: 0;
		top: 0;
		bottom: 0;
		width: .4em;
		right: 1rem;
	}


	a {
		color: black;
		cursor: pointer;
	}

	#information_box {
		background-color: rgba(255, 247, 248, 0.5);
		box-shadow: 10px 10px 16px 0px rgba(0, 0, 0, 0.75);
		width: 85vw;
		min-height: 600px;
		border-radius: 25px;
		text-align: center;
		font-family: "system-ui", sans-serif;
		font-size: 16px;
		color: black;
		margin: 50px auto 50px auto;
		padding: 0em 2em 2em 2em;
	}

	@media (max-width: 600px) {
		#information_box {
			width: 100vw;
			margin: 0 auto;
			border-radius: 0px;
			background-color: rgba(255, 247, 248, 0.35);
			padding: 0em 0em 2em 0em;
		}

	}


	.menuContainer {
		padding: 2em 0.2em 0em 0.2em;
	}

	.menuContainer_title {
		font-size: 36px;
		color: black;
		font-weight: bold;
	}

	.menuContainer_list {
		font-size: 26px;
	}

	.menuContainer_options {
		padding: 0;
		list-style-position: inside;
	}

	.contentContainer {
		padding: 0em 1em;
		text-align: justify;
	}

	.localization {
		width: 600px;
		height: 450px;
		border: 1px solid black;
	}

	.caption {
		font-weight: bold;
	}

	@media (max-width: 750px) {
		.localization {
			width: calc(3.2rem + 74.25vw);
			height: calc(4rem + 42.25vh);

		}
	}
</style>