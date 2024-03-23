function change_theme() {
    const body = document.body;
    const themeSwitcherIcon = document.getElementById('themeSwitcher').querySelector('span');
    if (body.dataset.bsTheme === "light") {
        body.dataset.bsTheme = "dark";
        themeSwitcherIcon.innerHTML = "&#9790;"; // Mesiacik ikonka pre tmave one
    } else {
        body.dataset.bsTheme = "light";
        themeSwitcherIcon.innerHTML = "&#9788;"; // slniecko ikonka
    }
}