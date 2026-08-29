from datetime import timedelta

from aligner.models.cue import Cue


LRC_CONTENTS = [
    Cue(
        start=timedelta(seconds=10, milliseconds=410),
        end=timedelta(seconds=12, milliseconds=920),
        content="Que si me escribes y no contesto",
    ),
    Cue(
        start=timedelta(seconds=12, milliseconds=920),
        end=timedelta(seconds=15, milliseconds=530),
        content="Tú no te ofendas si no aparezco",
    ),
    Cue(
        start=timedelta(seconds=15, milliseconds=530),
        end=timedelta(seconds=17, milliseconds=990),
        content="El sentimiento yo no lo presto",
    ),
    Cue(
        start=timedelta(seconds=17, milliseconds=990),
        end=timedelta(seconds=20, milliseconds=580),
        content="Es que no soy de nadie, no soy dueto",
    ),
    Cue(
        start=timedelta(seconds=20, milliseconds=580),
        end=timedelta(seconds=25, milliseconds=470),
        content="Please don't take it personal (nah, nah, nah, nah, nah, yeah)",
    ),
    Cue(
        start=timedelta(seconds=25, milliseconds=470),
        end=timedelta(seconds=30, milliseconds=450),
        content="If I don't check up here through the phone, woah",
    ),
    Cue(
        start=timedelta(seconds=30, milliseconds=450),
        end=timedelta(seconds=33, milliseconds=100),
        content="Me dicen bipolar",
    ),
    Cue(
        start=timedelta(seconds=33, milliseconds=100),
        end=timedelta(seconds=34, milliseconds=630),
        content="Porque un día quiero todo",
    ),
    Cue(
        start=timedelta(seconds=34, milliseconds=630),
        end=timedelta(seconds=40, milliseconds=640),
        content="Pero al otro ya no me interesa verte más, ah-ah",
    ),
    Cue(
        start=timedelta(seconds=40, milliseconds=640),
        end=timedelta(seconds=43, milliseconds=360),
        content="Me dicen bipolar",
    ),
    Cue(
        start=timedelta(seconds=43, milliseconds=360),
        end=timedelta(seconds=44, milliseconds=890),
        content="A veces fuego, a veces frío",
    ),
    Cue(
        start=timedelta(seconds=44, milliseconds=890),
        end=timedelta(seconds=46, milliseconds=180),
        content="A vecеs noche, a veces día",
    ),
    Cue(
        start=timedelta(seconds=46, milliseconds=180),
        end=timedelta(seconds=51, milliseconds=300),
        content="Pеro soy real, ah-ah",
    ),
    Cue(
        start=timedelta(seconds=51, milliseconds=300),
        end=timedelta(seconds=53, milliseconds=590),
        content="Vengo, me voy ya",
    ),
    Cue(
        start=timedelta(seconds=53, milliseconds=590),
        end=timedelta(seconds=56, milliseconds=60),
        content="Yo sé cómo soy, va",
    ),
    Cue(
        start=timedelta(seconds=56, milliseconds=60),
        end=timedelta(seconds=58, milliseconds=540),
        content="I know that I got issues, that's the truth",
    ),
    Cue(
        start=timedelta(seconds=58, milliseconds=540),
        end=timedelta(minutes=1, seconds=1, milliseconds=90),
        content="And if you get offended, that's on you",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=1, milliseconds=90),
        end=timedelta(minutes=1, seconds=3, milliseconds=790),
        content="Every emotion I felt that times two",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=3, milliseconds=790),
        end=timedelta(minutes=1, seconds=6, milliseconds=530),
        content="It hurts when you play to lose",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=6, milliseconds=530),
        end=timedelta(minutes=1, seconds=11, milliseconds=910),
        content="Been burned and I've been accused, yeah-yeah, yeh, yeh, eh",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=11, milliseconds=910),
        end=timedelta(minutes=1, seconds=14, milliseconds=460),
        content="Me dicen bipolar",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=14, milliseconds=460),
        end=timedelta(minutes=1, seconds=15, milliseconds=900),
        content="Porque un día quiero todo",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=15, milliseconds=900),
        end=timedelta(minutes=1, seconds=22, milliseconds=170),
        content="Pero al otro ya no me interesa verte más, ah-ah (yeh, yeh, yeh)",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=22, milliseconds=170),
        end=timedelta(minutes=1, seconds=24, milliseconds=690),
        content="Me dicen bipolar",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=24, milliseconds=690),
        end=timedelta(minutes=1, seconds=26, milliseconds=140),
        content="A veces fuego, a veces frío",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=26, milliseconds=140),
        end=timedelta(minutes=1, seconds=27, milliseconds=100),
        content="A veces noche, a veces día",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=27, milliseconds=100),
        end=timedelta(minutes=1, seconds=32, milliseconds=710),
        content="Pero soy real, ah-ah (nah, nah, nah, nah, nah, nah)",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=32, milliseconds=710),
        end=timedelta(minutes=1, seconds=36, milliseconds=690),
        content="Que si me escribes y no contesto",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=36, milliseconds=690),
        end=timedelta(minutes=1, seconds=39, milliseconds=910),
        content="Tú no te ofendas si no aparezco",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=39, milliseconds=910),
        end=timedelta(minutes=1, seconds=43, milliseconds=20),
        content="Mi sentimiento yo no lo presto",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=43, milliseconds=20),
        end=timedelta(minutes=1, seconds=46, milliseconds=700),
        content="Es que no soy de nadie, no soy dueto",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=46, milliseconds=700),
        end=timedelta(minutes=1, seconds=52, milliseconds=730),
        content="Please don't take it personal (nah, nah, nah, nah, nah)",
    ),
    Cue(
        start=timedelta(minutes=1, seconds=52, milliseconds=730),
        end=timedelta(minutes=2, seconds=0, milliseconds=50),
        content="If I don't check up here through the phone, woah",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=0, milliseconds=50),
        end=timedelta(minutes=2, seconds=3, milliseconds=100),
        content="Me dicen bipolar",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=3, milliseconds=100),
        end=timedelta(minutes=2, seconds=5, milliseconds=120),
        content="Porque un día quiero todo",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=5, milliseconds=120),
        end=timedelta(minutes=2, seconds=13, milliseconds=220),
        content="Pero al otro ya no me interesa verte más, ah-ah (yeh, yeh, yeh)",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=13, milliseconds=220),
        end=timedelta(minutes=2, seconds=16, milliseconds=850),
        content="Me dicen bipolar",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=16, milliseconds=850),
        end=timedelta(minutes=2, seconds=18, milliseconds=720),
        content="A veces fuego, a veces frío",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=18, milliseconds=720),
        end=timedelta(minutes=2, seconds=20, milliseconds=360),
        content="A veces noche, a veces día",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=20, milliseconds=360),
        end=timedelta(minutes=2, seconds=27, milliseconds=100),
        content="Pero soy real, ah-ah (nah, nah, nah, nah, nah, nah)",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=27, milliseconds=100),
        end=timedelta(minutes=2, seconds=30, milliseconds=590),
        content="Me dicen bipolar",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=30, milliseconds=590),
        end=timedelta(minutes=2, seconds=32, milliseconds=450),
        content="Porque un día quiero todo",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=32, milliseconds=450),
        end=timedelta(minutes=2, seconds=40, milliseconds=680),
        content="Pero al otro ya no me interesa verte más, ah, ah-ah (yeh, yeh, yeh)",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=40, milliseconds=680),
        end=timedelta(minutes=2, seconds=44, milliseconds=390),
        content="Me dicen bipolar",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=44, milliseconds=390),
        end=timedelta(minutes=2, seconds=46, milliseconds=50),
        content="A veces fuego, a veces frío",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=46, milliseconds=50),
        end=timedelta(minutes=2, seconds=47, milliseconds=840),
        content="A veces noche, a veces día",
    ),
    Cue(
        start=timedelta(minutes=2, seconds=47, milliseconds=840),
        end=timedelta(minutes=2, seconds=53, milliseconds=870),
        content="Pero soy real, ah-ah (nah, nah, nah, nah, nah, nah, nah)",
    ),
]
