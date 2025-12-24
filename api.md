
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
<string> // Le mot
```

### `word-hints`

Indices pour le dernier mot du joueur. Signale qu'il faut passer au tour suivant.

```json
[
    <green|yellow|red|gray> // Indice pour chaque letter
]
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