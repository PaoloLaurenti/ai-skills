# ai-skills

Le mie skill per gli agenti AI, tenute in versione qui invece che sparse nelle
directory di configurazione dei tool.

## Skill

| Skill | A cosa serve |
| --- | --- |
| [`paolo-prs-review`](skills/paolo-prs-review/SKILL.md) | Come voglio siano scritte e consegnate le review delle pull request: stato pending, tono, forma del commento generale, permalink, code suggestion. |

## Come è collegato a Claude Code

Claude Code carica le skill personali da `~/.claude/skills/`. Ogni skill di
questo repo è **symlinkata** lì dentro, quindi esiste una sola copia del file:
modificarla durante una sessione modifica direttamente il working tree del repo,
e resta solo da committare.

```
~/dev/personal/ai-skills/skills/paolo-prs-review/   <- la cartella vera
~/.claude/skills/paolo-prs-review                   -> symlink a quella sopra
```

## Setup su una macchina nuova

```bash
git clone git@github.com:PaoloLaurenti/ai-skills.git ~/dev/personal/ai-skills
ln -s ~/dev/personal/ai-skills/skills/paolo-prs-review ~/.claude/skills/paolo-prs-review
```

Una nuova sessione di Claude Code deve elencare la skill tra quelle disponibili.
Se non compare, il loader non sta seguendo il symlink: in quel caso l'alternativa
è rendere `~/.claude/skills` stessa un repo, tracciando solo le proprie skill con
un `.gitignore` ad allowlist.

## Aggiungere una skill

```bash
mkdir -p skills/<nome>
$EDITOR skills/<nome>/SKILL.md
ln -s ~/dev/personal/ai-skills/skills/<nome> ~/.claude/skills/<nome>
```

Il frontmatter richiede `name` e `description`. La `description` decide quando la
skill viene caricata, quindi descrive solo le condizioni di attivazione e non il
procedimento: se riassume il workflow, l'agente tende a seguire il riassunto
invece di leggere il corpo.
