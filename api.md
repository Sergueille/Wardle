
# API Websockets

Pour créer une nouvelle partie, le client doit ouvrir une connection websocket sur l'adresse: `/create-room`.
Pour rejoindre, il doit se connecter sur `/join-room/<code>`. 

Les messages sont encodés en JSON et sont toujours de la forme

```json
{
    "type": <string>, // Type du message
    "content": <object> // Contenu du message
}
```

Pour chaque entée, seulement le contenu du message est indiqué (le type de message est le titre de chaque entrée).

## Serveur vers client

### `room-code`

Envoyé au premier joueur qui rejoint la salle

```json
<string> // Le code de la salle
```

### `other-player-info`

Indique des informations sur l'autre joueur.

```json
{
    "name": <string> // Nom du joueur
}
```

### `other-player-connected`

Indique que l'autre joueur est connecté.

*Pas de contenu*

### `word-rejected`

Le mot envoyé pour ce tour est incorrect

*Pas de contenu*

### `other-player-word`

Mot qu'a écrit l'autre joueur. Signale qu'il faut passer à la phase de sabotage.

```json
{
    "word": <string>, // Le mot
    "who_wins": <none|you|other|both>, // Si quelqu'un gagne ce tour ci (en tout cas moi je perds)
}
```

### `word-hints`

Indices pour le dernier mot du joueur. Signale qu'il faut passer au tour suivant.

```json
[
    <green|yellow|red|gray> // Indice pour chaque letter
]
```

### `solution`

Solution de la partie en cours. Indique également que les deux joueurs ont perdu. (et moi aussi)

```
<string>
```

### `restart`

La partie recommence

*Pas de contenu*

### `wait-for-host`

Le joueur doit attendre l'hôte

*Pas de contenu*

### `other-player-is-done`

L'autre joueur a fini son action

*Pas de contenu*

### `game-options`

Options pour la partie (envoyé seulement au joueur non hote)

```json
{
    "options": <GameOptions> // Correspond au type option dans `main.rs`
}
```

## Client vers serveur

### `ping`

*Pas de contenu*

### `word`

Le joueur a entré un nouveau mot

```json
{
    "word": <string>
}
```

### `sabotage`

Le joueur a saboté une lettre

```json
{
    "id": <string> // Indice de la letter sabotée
}
```

### `restart-ready`

Le joueur est prêt a recommencer (ou a commencer)

```json
{}
```

### `game-options`

Mise a jour des options pour la partie.

```json
{
    "options": <GameOptions> // Correspond au type option dans `main.rs`
}
```
