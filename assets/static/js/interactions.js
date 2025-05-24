// Mobile Burger menu
document.addEventListener("DOMContentLoaded", () => {
  const triggerButton = document.getElementById("home-burger-menu-trigger");
  const sideMenu = document.getElementById("side-menu");
  const menuOverlay = document.getElementById("menu-overlay");
  const closeButton = document.getElementById("close-menu-button");

  let isMenuOpen = false;

  function toggleMenu() {
    isMenuOpen = !isMenuOpen;
    if (isMenuOpen) {
      sideMenu.style.transform = "translateX(0)";
      if (menuOverlay) {
        menuOverlay.style.opacity = "1";
        menuOverlay.style.pointerEvents = "auto";
      }
      document.body.style.overflow = "hidden";
    } else {
      sideMenu.style.transform = "translateX(100%)";
      if (menuOverlay) {
        menuOverlay.style.opacity = "0";
        menuOverlay.style.pointerEvents = "none";
      }
      document.body.style.overflow = "";
    }
  }

  if (triggerButton) {
    triggerButton.addEventListener("click", toggleMenu);
  }
  if (closeButton) {
    closeButton.addEventListener("click", toggleMenu);
  }
  if (menuOverlay) {
    menuOverlay.addEventListener("click", toggleMenu);
  }
  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape" && isMenuOpen) {
      toggleMenu();
    }
  });
});

// Scroll to anchor
document.body.addEventListener("htmx:afterSwap", function (evt) {
  if (evt.detail.target.id === "app") {
    const target = sessionStorage.getItem("scrollTo");
    if (target) {
      requestAnimationFrame(() => {
        const el = document.querySelector(target);
        if (el) {
          const elementRect = el.getBoundingClientRect();
          const offsetPosition =
            elementRect.top + window.pageYOffset - window.innerHeight / 2 + elementRect.height / 2;
          window.scrollTo({
            top: offsetPosition,
            behavior: "smooth",
          });
        }
        sessionStorage.removeItem("scrollTo");
      });
    }
  }
});
