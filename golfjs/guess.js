guesses = 1
correct = ~~(Math.random() * 100) + 1;
console.log(correct)
while((guess = +prompt()) != correct) {
    guesses++;
    if (correct > guess) console.log('greater')
    if (correct < guess) console.log('less')
}
console.log(guesses)

t = 1
c = ~~(Math.random() * 100) + 1;
console.log(c)
while((g = +prompt()) != c) 
    t++,
    c > g && console.log('greater'),
    c < g && console.log('less');
console.log(t)

for (
    c = ~~(Math.random() * 100) + 1, t = 1;
    (g = +prompt()) != c;
    t++,
    c > g && console.log('greater'),
    c < g && console.log('less')
);console.log(t)

for(c=~~(Math.random()*100)+1,t=1;(g=+prompt())!=c;t++,c>g&&console.log('greater'),c<g&&console.log('less'));console.log(t)

for(l=prompt,c=~~(Math.random()*100)+1,t=1;(g=+l())!=c;t++,c>g&&l(1),c<g&&l(-1));l(t)

for(c=~~(Math.random(l=prompt)*100)+1,t=1;(g=+l())!=c;t++,c>g&&l(1),c<g&&l(0));l(t)

for(l=prompt,c=1+new Date%100,t=1;(g=+l())!=c;t++,c>g&&l(1),c<g&&l(0));l(t)

for(l=prompt,c=1+new Date%100,t=1;(g=+l())-c;t++,c>g&&l(1),c<g&&l(0));l(t)
