(function () {
  /* ========================
     CONTEXT
  ======================== */
  // Récupération du script courant pour d'éventuelles configurations futures
  var loginScript = document.currentScript;

  /* ========================
     SELECTORS
  ======================== */
  // Helper pour sélectionner un élément à partir d'une racine donnée
  var select = function (element, all) {
    return function (selector) {
      return element[all ? "querySelectorAll" : "querySelector"](selector);
    };
  };

  /* ========================
     METHODS
  ======================== */
  // Envoie la requête POST vers l'API /auth/login avec les données fournies
  // L'option "credentials: 'same-origin'" permet de gérer l'envoi et la réception des cookies
  var requestLogin = function (payload) {
    return fetch("/auth/login", {
      method: "POST",
      credentials: "same-origin",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(payload),
    });
  };

  function bindLoginEvents(opts) {
    var target = opts.target;
    var loginForm = target.querySelector("#" + opts.formId);
    var errorBanner = target.querySelector("#" + opts.errorBannerId);

    if (!loginForm) {
      console.error("[Nouvelle Lettre] Login form not found!");
      return;
    }

    loginForm.addEventListener("submit", function (event) {
      event.preventDefault();

      // Récupération des valeurs du formulaire
      var email = loginForm.querySelector('input[name="email"]').value;
      var password = loginForm.querySelector('input[name="password"]').value;

      // Réinitialisation de la bannière d'erreur
      if (errorBanner) {
        errorBanner.style.display = "none";
        errorBanner.textContent = "";
      }

      // Envoi de la requête de login vers l'API
      requestLogin({ email: email, password: password })
        .then(function (response) {
          // Si l'API redirige, suivre la redirection
          if (response.redirected) {
            window.location.href = response.url;
            return;
          }
          if (response.ok) {
            // En cas de succès, le cookie d'authentification est reçu et on redirige vers l'application
            window.location.href = opts.successRedirect;
          } else {
            // Afficher l'erreur renvoyée par l'API
            response.json().then(function (data) {
              if (errorBanner) {
                errorBanner.textContent =
                  data.message || "Erreur lors de la connexion.";
                errorBanner.style.display = "block";
              }
            });
          }
        })
        .catch(function (error) {
          console.error("[Nouvelle Lettre] Error during login request:", error);
          if (errorBanner) {
            errorBanner.textContent =
              "Erreur de connexion, veuillez réessayer plus tard.";
            errorBanner.style.display = "block";
          }
        });
    });
  }

  /* ========================
     OPTIONS
  ======================== */
  // Options de configuration pour Nouvelle Lettre
  var options = {
    target:
      document.querySelector("[data-nouvelle-lettre-target]") || document.body,
    formId: "nouvelle-lettre-loginForm",
    errorBannerId: "nouvelle-lettre-errorBanner",
    successRedirect: "/", // Redirection en cas de succès
  };

  /* ========================
     INITIALIZE
  ======================== */
  if (options.target) {
    bindLoginEvents(options);
  } else {
    console.error(
      "[Nouvelle Lettre] Could not initialize: does nouvelle-lettre-target exist?"
    );
  }
})();
