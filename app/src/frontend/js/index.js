function setCookie(name, value, days) {
    const d = new Date();
    d.setTime(d.getTime() + (days * 24 * 60 * 60 * 1000));
    const expires = "expires=" + d.toUTCString();
    document.cookie = name + "=" + value + ";" + expires + ";path=/";
}

function getCookie(name) {
    const nameEQ = name + "=";
    const ca = document.cookie.split(';');
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i];
        while (c.charAt(0) === ' ') c = c.substring(1, c.length);
        if (c.indexOf(nameEQ) === 0) return c.substring(nameEQ.length, c.length);
    }
    return null;
}

function change_theme() {
    const body = document.body;
    const themeSwitcherIcon = document.getElementById('themeSwitcher').querySelector('span');
    if (body.dataset.bsTheme === "light") {
        body.dataset.bsTheme = "dark";
        themeSwitcherIcon.innerHTML = "&#9790;"; // Mesiacik ikonka pre tmave one
        setCookie("theme", "dark", 365); // Save 'dark' theme in cookies for 365 days
    } else {
        body.dataset.bsTheme = "light";
        themeSwitcherIcon.innerHTML = "&#9788;"; // slniecko ikonka
        setCookie("theme", "light", 365); // Save 'light' theme in cookies for 365 days
    }
}

function applySavedTheme() {
    const savedTheme = getCookie("theme");
    const body = document.body;
    const themeSwitcherIcon = document.getElementById('themeSwitcher').querySelector('span');
    if (savedTheme) {
        body.dataset.bsTheme = savedTheme;
        themeSwitcherIcon.innerHTML = savedTheme === "dark" ? "&#9790;" : "&#9788;";
    } else {
        body.dataset.bsTheme = "light"; // Default to light theme
        themeSwitcherIcon.innerHTML = "&#9788;";
    }
}

document.addEventListener("DOMContentLoaded", applySavedTheme);
