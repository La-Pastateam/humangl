# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: tcros <tcros@student.42.fr>                +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2026/06/16 11:49:09 by tcros             #+#    #+#              #
#    Updated: 2026/10/21 12:45:41 by tcros            ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

### VARIABLES ###
# NOM #
EXEC	= humangl

# COMPILATION #
CMPL	= cargo
MODE 	?= dev

ifeq ($(MODE),release)
	RFLAG	= --release
	BIN_DIR	= target/release
else ifeq ($(MODE),dev)
	RFLAG	=
	BIN_DIR	= target/debug
else
	$(error MODE invalide : "$(MODE)". Valeurs possibles : dev, release)
endif

### COULEURS ###
GREEN	= \033[3;32m
RED		= \033[0;31m
ORANGE	= \033[1;33m
BLUE	= \033[0;34m
NC		= \033[0m

### REGLES D'INFERENCES ###
# REGLES PHONY #
.PHONY: all build compile-shaders run test clippy fix help clean fclean re 

ifeq ($(MODE),release)
	all: $(EXEC)
else
	all: build
endif

all: $(EXEC)

$(EXEC): build
	@printf "$(BLUE)[INFO] Copie de l'exécutable $(EXEC) à la racine...$(NC)\n"
	@cp -u $(BIN_DIR)/$(EXEC) ./$(EXEC)
	@printf "$(GREEN)[SUCCÈS] $(EXEC) compilé avec succès ($(MODE)). ✅\n$(NC)"

build: compile-shaders
	@printf "$(BLUE)[INFO] Build de $(EXEC) en mode $(MODE)...$(NC)\n"
	@$(CMPL) build $(RFLAG)

compile-shaders:
	@printf "$(BLUE)[INFO] Compilation des shaders...$(NC)\n"
	@cd shaders && ./compile.sh
	@printf "$(GREEN)[SUCCÈS] Compilation des shaders terminée !$(NC)\n"

run:
	@printf "$(ORANGE)[INFO] Run direct de $(EXEC) en mode $(MODE)...$(NC)\n"
	@$(CMPL) run $(RFLAG)

test:
	@printf "$(BLUE)[INFO] Test du projet $(EXEC)...$(NC)\n"
	@$(CMPL) test $(RFLAG)

clippy:
	@printf "$(BLUE)[INFO] Analyse Clippy de $(EXEC)...$(NC)\n"
	$(CMPL) clippy $(RFLAG) -- -D warnings

fix:
	@printf "$(BLUE)[INFO] Fix par le compilateur du projet $(EXEC)...$(NC)\n"
	@printf "$(ORANGE)Le fix modifie automatiquement les fichiers concernés.$(NC)\n"
	@read -p "Voulez-vous procéder ? [y/N] : " ans && [ "$${ans:-N}" = y ] || (echo "Opération annulée."; exit 1)
	@$(CMPL) clippy --fix --allow-dirty
	@printf "$(GREEN)Fix effectué avec succès !$(NC)\n"

help:
	@echo "Usage : make [règle] [MODE=dev|release]"
	@echo ""
	@echo "Règles disponibles :"
	@echo "  make / make all  - Compile le projet (MODE=release : copie aussi l'exécutable $(EXEC) à la racine)"
	@echo "  make $(EXEC)        - Compile et copie l'exécutable $(EXEC) à la racine"
	@echo "  make build       - Compile le projet sans copier l'exécutable"
	@echo "  make run         - Compile et exécute le projet"
	@echo "  make test        - Lance les tests"
	@echo "  make clippy      - Lance le linter Clippy (warnings traités comme erreurs)"
	@echo "  make fix         - Applique les corrections automatiques de Clippy (avec confirmation)"
	@echo "  make clean       - Supprime le dossier target/"
	@echo "  make fclean      - clean + supprime l'exécutable $(EXEC)"
	@echo "  make re          - fclean puis all"
	@echo "  make help        - Affiche cette aide"
	@echo ""
	@echo "Modes (défaut : dev) :"
	@echo "  MODE=dev         - Build debug, rapide à compiler, non optimisé"
	@echo "  MODE=release     - Build optimisé + exécutable $(EXEC) à la racine"
	@echo ""
	@echo "Exemples :"
	@echo "  make MODE=release"
	@echo "  make run "
	@echo "  make re MODE=release"

clean:
	@$(CMPL) clean
	@printf "$(ORANGE)[CLEAN] Fichiers target supprimés.$(NC)\n"

fclean: clean
	@rm -f $(EXEC)
	@printf "$(RED)[FCLEAN] binaire supprimé.$(NC)\n"

re: fclean all
