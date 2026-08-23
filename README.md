# TSE-Verifikation

Universitätsprojekt in Rust: prüft die Angaben einer TSE-QR-Nachricht (Kassenbon). Verifiziert werden der öffentliche Schlüssel (ECC-Pubkey) und die ECDSA-Signatur auf den Kurven NIST P-384 (secp384r1) und brainpoolP384r1. Zur Kontrolle der Ergebnisse liegen zwei PARI/GP-Skripte bei.

## Benutzung

Die zu prüfende QR-Nachricht steht als `DATA`-String in `src/main.rs`. Aktuell ist dort eine Beispiel-Nachricht hinterlegt. Den Inhalt des Strings durch die eigene QR-Nachricht ersetzen und das Programm ausführen:

```sh
cargo run
```

Alternativ lässt sich das Projekt aus einer IDE wie RustRover oder Visual Studio Code heraus starten.

## Überprüfung des öffentlichen Schlüssels

Die Koordinaten des PubKeys werden auf beiden Kurven geprüft. Die folgende PARI/GP-Session zeigt das:

```gp
? \\ 1. Koordinaten als Hexadezimalzahlen definieren
? dx = 0x80991a288ea13cc52f41a835dc1f929be1ba8e7a6cd5f37a46da71e789081fc5b5d0f93c58d3577fd6aac70d4d28effa;
? dy = 0x58600414a3b3406bc4d75609f5c5fd1f0b4c598631f0f582ab0713cbbfadd36532d9c45ea701a39ff313fa0af2dd825e;
? 
? \\ 2. Prüfung für secp384r1 (NIST P-384)
? p_secp = 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff;
? a_secp = -3;
? b_secp = 0xb3312fa7e23ee7e4988e056be3f82d19181d9c6efe8141120314088f5013875ac656398d8a2ed19d2a85c8edd3ec2aef;
? E_secp = ellinit([a_secp, b_secp], p_secp);
? print("Punkt auf secp384r1? ", ellisoncurve(E_secp, [dx, dy]));
Punkt auf secp384r1? 0
? 
? \\ 3. Prüfung für brainpoolP384r1
? p_bp = 0x8cb91e82a3386d280f5d6f7e50e641df152f7109ed5456b412b1da197fb71123acd3a729901d1a71874700133107ec53;
? a_bp = 0x7bc382c63d8c150c3c72080ace05afa0c2bea28e4fb22787139165efba91f90f8aa5814a503ad4eb04a8c7dd22ce2826;
? b_bp = 0x4a8c7dd22ce28268b39b55416f0447c2fb77de107dcd2a62e880ea53eeb62d57cb4390295dbc9943ab78696fa504c11;
? E_bp = ellinit([a_bp, b_bp], p_bp);
? print("Punkt auf brainpoolP384r1? ", ellisoncurve(E_bp, [dx, dy]));
Punkt auf brainpoolP384r1? 1
?
```

Der PubKey liegt demnach auf brainpoolP384r1, nicht auf secp384r1.

## Signaturprüfung

Die ECDSA-Signatur wird auf brainpoolP384r1 geprüft. Das Skript berechnet den SHA-384-Hash der Log-Nachricht, wendet die ECDSA-Verifikation an und vergleicht die x-Koordinate des Ergebnisses mit `r`:

```gp
p = 0x8cb91e82a3386d280f5d6f7e50e641df152f7109ed5456b412b1da197fb71123acd3a729901d1a71874700133107ec53;
a = 0x7bc382c63d8c150c3c72080ace05afa0c2bea28e4fb22787139165efba91f90f8aa5814a503ad4eb04a8c7dd22ce2826;
b = 0x4a8c7dd22ce28268b39b55416f0447c2fb77de107dcd2a62e880ea53eeb62d57cb4390295dbc9943ab78696fa504c11;
n = 0x8cb91e82a3386d280f5d6f7e50e641df152f7109ed5456b412b1da197fb71123acd3a7292721921fd81d0130c0b7d0d;
G = [0x1d1c64f143232dd4278ad3511453dd036936a234b39bb307943f74c0534226190772410a568285558197771764c20790, 0x5ad147f154edb176e58319695d73010b0a8849646b1064d13f04495e26343513b827725965476a6642d96c99ec9e88];

\\ Initialisierung der Kurve
E = ellinit([a, b], p);

\\ Öffentlicher Schlüssel (PubKey)
Dx = 0x80991a288ea13cc52f41a835dc1f929be1ba8e7a6cd5f37a46da71e789081fc5b5d0f93c58d3577fd6aac70d4d28effa;
Dy = 0x58600414a3b3406bc4d75609f5c5fd1f0b4c598631f0f582ab0713cbbfadd36532d9c45ea701a39ff313fa0af2dd825e;
D = [Dx, Dy];

\\ Signatur r und s (jeweils 48 Bytes / 96 Hex-Zeichen)
r = 0x77687BCCB39CDFDBEC746F49CDB566424831A652883DD7999BEC160AD3451A71581B5C7D6DEEEF5B18654D;
s = 0xF2417901E85A6BEA0CEEAFC08A47C468D6FDC3E43256DB6FD0E74FF98781363C86A1F7842CFA52A7FC530E2160A7E8359DCF45C049;

\\ Berechneter SHA384 Hash der Log-Message
h = 0xB7BBFDA54DAC93AF2795AF6F1B415AA0541082F46AFFBFA625A2258168D7C37DCD9ADA684276DD8A8F698C771CF2;

w = Mod(1/s, n);
u1 = lift(h * w);
u2 = lift(r * w);

\\ Punktaddition und Skalarmultiplikation auf der Kurve
L = elladd(E, ellpow(E, G, u1), ellpow(E, D, u2));

\\ Vergleich der x-Koordinate von L mit r
print("Signatur gültig? ", Mod(L[1], n) == r);

Signatur gültig? 1
```

## Danksagung

Vielen Dank an Martin für seine Unterstützung bei diesem Projekt.
