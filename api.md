
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

## Clint vers serveur

### `ping`

*Pas de contenu*


