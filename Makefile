# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: tcros <tcros@student.42.fr>                +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2026/06/16 11:49:09 by tcros             #+#    #+#              #
#    Updated: 2026/10/27 17:34:33 by nbacconn         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #
### VARIABLES ###
# NOM #
EXEC       = humangl
IMAGE_NAME = rust-vulkan-env

# COMPILATION #
RUNNER   = ./podman/run.sh
CMPL     = cargo
MODE     ?= dev

ifeq ($(MODE),release)
    RFLAG    = --release
    BIN_DIR    = target/release
else ifeq ($(MODE),dev)
    RFLAG    =
    BIN_DIR    = target/debug
else
    $(error MODE invalide : "$(MODE)". Valeurs possibles : dev, release)
endif

### COULEURS ###
GREEN    = \033[3;32m
RED        = \033[0;31m
ORANGE    = \033[1;33m
BLUE    = \033[0;34m
NC        = \033[0m

### REGLES D'INFERENCES ###
# REGLES PHONY #
.PHONY: all build compile-shaders run test clippy fix help clean fclean re image image-clean 

ifeq ($(MODE),release)
    all: $(EXEC)
else
    all: build
endif

all: $(EXEC)

$(EXEC): build run
	@printf "$(BLUE)[INFO] Copie de l'exécutable $(EXEC) à la racine...$(NC)\n"
	@printf "$(GREEN)[SUCCÈS] $(EXEC) compilé avec succès ($(MODE)). ✅\n$(NC)"

build: compile-shaders
	@printf "$(BLUE)[INFO] Build de $(EXEC) en mode $(MODE) (via Podman)...$(NC)\n"
	@$(RUNNER) $(CMPL) build $(RFLAG)

compile-shaders:
	@printf "$(BLUE)[INFO] Compilation des shaders (via Podman)...$(NC)\n"
	@$(RUNNER) bash -c "cd shaders && ./compile.sh"
	@printf "$(GREEN)[SUCCÈS] Compilation des shaders terminée !$(NC)\n"

run:
	@printf "$(ORANGE)[INFO] Run de $(EXEC) en mode $(MODE) (via Podman)...$(NC)\n"
	@$(RUNNER) $(CMPL) run $(RFLAG)

test:
	@printf "$(BLUE)[INFO] Test du projet $(EXEC) (via Podman)...$(NC)\n"
	@$(RUNNER) $(CMPL) test $(RFLAG)

clippy:
	@printf "$(BLUE)[INFO] Analyse Clippy de $(EXEC) (via Podman)...$(NC)\n"
	@$(RUNNER) $(CMPL) clippy $(RFLAG) -- -D warnings

fix:
	@printf "$(BLUE)[INFO] Fix par le compilateur du projet $(EXEC)...$(NC)\n"
	@printf "$(ORANGE)Le fix modifie automatiquement les fichiers concernés.$(NC)\n"
	@read -p "Voulez-vous procéder ? [y/N] : " ans && [ "$${ans:-N}" = y ] || (echo "Opération annulée."; exit 1)
	@$(RUNNER) $(CMPL) clippy --fix --allow-dirty
	@printf "$(GREEN)Fix effectué avec succès !$(NC)\n"

image:
	@printf "$(BLUE)[INFO] Construction de l'image Podman $(IMAGE_NAME)...$(NC)\n"
	@podman build -t $(IMAGE_NAME) -f podman/Containerfile .
	@printf "$(GREEN)[SUCCÈS] Image Podman construite avec succès !$(NC)\n"

image-clean:
	@printf "$(RED)[INFO] Suppression de l'image Podman $(IMAGE_NAME)...$(NC)\n"
	@podman rmi -f $(IMAGE_NAME) 2>/dev/null || true
	@printf "$(GREEN)[SUCCÈS] Image Podman supprimée.$(NC)\n"

help:
	@echo "Usage : make [règle] [MODE=dev|release]"
	@echo ""
	@echo "Règles disponibles :"
	@echo "  make / make all  - Compile le projet dans Podman (MODE=release : copie aussi l'exécutable $(EXEC) à la racine)"
	@echo "  make $(EXEC)        - Compile et copie l'exécutable $(EXEC) à la racine"
	@echo "  make build       - Compile le projet dans Podman sans copier l'exécutable"
	@echo "  make run         - Compile et exécute le projet dans le conteneur Podman (avec GPU et affichage)"
	@echo "  make test        - Lance les tests dans Podman"
	@echo "  make clippy      - Lance le linter Clippy dans Podman (warnings traités comme erreurs)"
	@echo "  make fix         - Applique les corrections automatiques de Clippy (avec confirmation)"
	@echo "  make image       - Construit l'image Podman (Containerfile)"
	@echo "  make image-clean - Supprime l'image Podman du système"
	@echo "  make clean       - Supprime le dossier target/ via Podman"
	@echo "  make fclean      - clean + supprime l'exécutable $(EXEC) et l'image Podman"
	@echo "  make re          - fclean puis all"
	@echo "  make help        - Affiche cette aide"
	@echo ""
	@echo "Modes (défaut : dev) :"
	@echo "  make MODE=dev         - Build debug, rapide à compiler, non optimisé"
	@echo "  make MODE=release     - Build optimisé + exécutable $(EXEC) à la racine"
	@echo ""
	@echo "Exemples :"
	@echo "  make MODE=release"
	@echo "  make run "
	@echo "  make re MODE=release"

clean:
	@$(RUNNER) $(CMPL) clean
	@printf "$(ORANGE)[CLEAN] Fichiers target supprimés.$(NC)\n"

fclean: clean image-clean
	@rm -f $(EXEC)
	@printf "$(RED)[FCLEAN] binaire supprimé.$(NC)\n"

re: fclean all
