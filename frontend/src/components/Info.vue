<template>
	<main>

		<transition name="fade">
			<section v-if="loaded">

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

					<article id="informations"><a id="informacje"></a>
						<div class="heading_box">
							<h2 class="left"></h2>
							<h2 class="heading" style="">
								<div class="heading_text informations">Informacje</div>
							</h2>
							<h2 class="right"></h2>
						</div>
						<div class="content_box">
							Etiam tristique orci a ligula vestibulum placerat. Aliquam vitae lorem interdum,
							imperdiet purus id, consectetur ipsum. Duis et elit neque. Duis euismod, leo sit amet
							volutpat pellentesque, turpis turpis rutrum dui, in commodo magna magna eu justo.
							Suspendisse ornare dui vitae urna lobortis, et dignissim nisi porttitor. Aliquam
							mattis pharetra tellus vestibulum tincidunt. Ut eget imperdiet augue.
							<br><br>
							In mattis eget orci et suscipit. Pellentesque dapibus, urna ut dictum tincidunt, metus
							eros condimentum urna, sit amet aliquam diam turpis at enim. Aenean auctor dignissim
							efficitur. Donec facilisis urna non finibus vehicula. Cras auctor eget sem nec
							aliquam. Sed cursus commodo tellus, vel consequat mauris auctor in. Sed dui neque,
							tincidunt finibus pellentesque quis, pellentesque a velit.
						</div>
					</article>

					<article id="rules"><a id="regulamin"></a>
						<div class="heading_box">
							<h2 class="left"></h2>
							<h2 class="heading" style="">
								<div class="heading_text rules">Regulamin</div>
							</h2>
							<h2 class="right"></h2>
						</div>
						<div class="content_box">
							§1<br>
							Zasady ogólne <br>
							1. Studniówka jest uroczystością wynikającą z tradycji polskiej edukacji. <br>
							2. Regulamin obowiązuje wszystkich uczestników studniówki.<br>
							3. Szkoła będzie współdziałać z Komitetem w sprawie upowszechnienia regulaminu
							studniówki.<br>
							4. W celu organizacji Studniówki został powołany na czas określony Komitet
							Organizacyjny<br>
							Studniówki zwany dalej Komitetem Organizacyjnym w skład którego wchodzą
							przedstawiciele
							wyłonieni spośród rodziców uczniów klas maturalnych ( załącznik nr 4 ).<br>
							5. Uczniowie wraz z osobami towarzyszącymi oraz rodzice potwierdzają własnoręcznym
							podpisem
							akceptację Regulaminu (załącznik nr1,2, 3). Brak akceptacji wyklucza możliwość
							uczestnictwa w
							Studniówce.<br>
							6. W sytuacjach nie objętych poniższymi postanowieniami obowiązują przepisy prawa
							powszechnie
							obowiązującego oraz Kodeksu Cywilnego. <br>
						</div>
					</article>

					<article id="location" style="text-align:center;"><a id="lokalizacja"></a>
						<div class="heading_box">
							<h2 class="left"></h2>
							<h2 class="heading" style="">
								<div class="heading_text location">Lokalizacja</div>
							</h2>
							<h2 class="right"></h2>
						</div>
						<iframe class="localization"
										src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d2582.6872930508157!2d21.77954250978024!3d49.66019104553675!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x473c46253abe1567%3A0xc8612fafb0d56ccb!2sKompleks%20Rekreacyjno%20-%20Rozrywkowy%20DWA%20-%20SERCA!5e0!3m2!1spl!2spl!4v1583330882011!5m2!1spl!2spl"
										allowfullscreen=""></iframe>
						<div class="caption">Miejsce imprezy - "Dwa Serca"</div>
					</article>

					<article id="gallery"><a id="galeria"></a>
						<div class="heading_box">
							<h2 class="left"></h2>
							<h2 class="heading" style="">
								<div class="heading_text gallery">Galeria</div>
							</h2>
							<h2 class="right"></h2>
						</div>
						<div id="gallery">
							<transition-group name="thumbnailfade" tag="div">
								<img
										v-for="thumb in filteredImages" :key="thumb.id"
										@click="showLightbox(thumb.name)" :src="thumb.name" :alt="thumb.alt"
										:title="thumb.alt"/>
							</transition-group>

							<lightbox ref="lightbox"
												:images="images"
												:timeoutDuration="3000"
							></lightbox>
						</div>
					</article>

				</div>
			</section>
		</transition>
	</main>
</template>

<script>

	var imageList = [{
		'name': require("../assets/imgs/outside.jpg"),
		'alt': 'Budynek z zewnątrz',
		'id': 'image1'
	},
		{'name': require("../assets/imgs/inside.jpg"), 'alt': 'Środek sali', 'id': 'image2'},
		{'name': require("../assets/imgs/corridor.jpg"), 'alt': 'Korytarz', 'id': 'image3'},
		{
			'name': require("../assets/imgs/desserts.jpg"),
			'alt': 'Przykładowe desery',
			'id': 'image4'
		},
		{
			'name': require("../assets/imgs/beverages.jpg"),
			'alt': 'Stół z napojami',
			'id': 'image5'
		}];

	export default {
		name: 'Info',
		data() {
			return {
				images: imageList,
				galleryFilter: 'all',
				loaded: false,
			}
		},

		mounted: function () {
			this.loaded = true;
		},

		methods: {
			showLightbox: function (imageName) {
				this.$refs.lightbox.show(imageName);
			},
		},
		computed: {
			filteredImages: function () {
				if (this.galleryFilter === 'all') return this.images;
				else return this.images.filter(image => image.filter === this.galleryFilter);
			}
		}
	}
</script>

<style lang="scss" scoped>

	$color: white;

	/* Sekcja */

	section {
		background-color: rgba(15, 15, 15, 0.35);
		width: 85vw;
		min-height: 600px;
		border-radius: 25px;
		text-align: center;
		font-family: "system-ui", sans-serif;
		font-size: 16px;
		color: $color;
		margin: 50px auto 50px auto;
		padding: 0em 2em 2em 2em;

		/* text-shadow: -1px -1px 0 rgba(0,0,0,0.25), 1px -1px 0 rgba(0,0,0,0.25), -1px 1px 0 rgba(0,0,0,0.25), 1px 1px 0 rgba(0,0,0,0.25); */
	}

	@media (max-width: 600px) {
		section {
			width: 100vw;
			margin: 0 auto;
			border-radius: 0px;
			padding: 0em 0em 2em 0em;
		}

	}

	/* Menu sekcji */

	.menuContainer {
		padding: 2em 0.2em 0em 0.2em;
	}

	.menuContainer_title {
		font-size: 36px;
		font-weight: bold;
	}

	.menuContainer_list {
		font-size: 26px;
	}

	.menuContainer_options {
		padding: 0;
		list-style-position: inside;
	}

	a {
		color: $color;
		cursor: pointer;
	}

	/* Główna zawartość sekcji */

	.contentContainer {
		padding: 0em 1em;
		text-align: justify;
	}

	.content_box {
		padding: 0em 2em 0em 2em;
	}

	article {
		margin-bottom: 20px;
	}

	/* Elementy artykułu lokalizacja */

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

	/* Tytuł Artykułu */

	.heading {
		position: relative;
		text-align: center;
	}

	.heading_box {
		display: flex;
		align-items: center;
		justify-content: center;
		margin-bottom: 20px;
	}

	.left {
		content: " ";
		border-top: solid 5px white;
		width: 100vw;
	}

	.right {
		content: " ";
		border-top: solid 5px white;
		width: 100vw;
	}

	.informations {
		width: 185px;
	}

	.rules {
		width: 180px;
	}

	.location {
		width: 185px;
	}

	.gallery {
		width: 120px;
	}

	.heading_text {
		position: relative;
		display: inline-block;
		padding: .2em 2em;
		font-size: 36px;
	}

	.heading_text:before {
		content: " ";
		border: solid 5px white;
		border-right: 0px;
		position: absolute;
		left: 1rem;
		top: 0;
		bottom: 0;
		width: .4em;
		right: 0;
		margin-left: -16px;
	}

	.heading_text:after {
		content: "";
		border: solid 5px white;
		border-left: 0px;
		position: absolute;
		right: 0;
		top: 0;
		bottom: 0;
		width: .4em;
		right: 1rem;
		margin-right: -16px;
	}

	@media (max-width: 600px) {
		.heading_text {
			font-size: 30px;
			padding: .2em 1em;
		}
	}

	/* Animacje */

	.fade-enter-active {
		transition: opacity 0.5s ease-in-out;
	}

	.fade-enter-to {
		opacity: 1;
	}

	.fade-enter {
		opacity: 0;
	}

	/* Lightbox Galeria */

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

	.thumbnailfade-leave-active, .thumbnailfade-enter-active {
		transition: all 0.4s ease;
	}

	.thumbnailfade-enter-active {
		transition-delay: 0.4s;
	}

	.thumbnailfade-enter, .thumbnailfade-leave-to {
		opacity: 0;
	}

</style>