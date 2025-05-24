function updatePageTitle(title) {
  document.title = title;
}

function shiningBtn(compId) {
  const button = document.getElementById(compId);
  if (button) {
    const shineAnimationDuration = 1000;
    const intervalDuration = 8000;
    function triggerShine() {
      if (!button.classList.contains("animate-shine-active")) {
        button.classList.add("animate-shine-active");
        setTimeout(() => {
          button.classList.remove("animate-shine-active");
        }, shineAnimationDuration);
      }
    }

    const shineInterval = setInterval(triggerShine, intervalDuration);
  } else {
    console.error("Button with id 'shiningButton' not found.");
  }
}
