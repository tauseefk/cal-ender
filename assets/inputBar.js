const  KEYCODES = {
        "Enter": 13,
};

const load = () => {
        const urlInput = document.getElementById("url-input");
        urlInput.addEventListener("keydown", (e) => {
        e.stopPropagation();

        const target = e.target;
        const value = target.value;

                if (e.keyCode === KEYCODES["Enter"]) {
                        try {
                                const url = new URL(value);
                                urlInput.classList.remove("invalid");
                                window.location.href = `/read?url=${url.href}`;
                        }
                        catch (e) {
                                urlInput.classList.add("invalid");
                        }
                }
        })
}
window.addEventListener("DOMContentLoaded", load);

