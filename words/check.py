def checkW(w, tab) :
    if len(tab) == 0:
        return False
    if len(tab) == 1:
        return tab[0] == w
    
    m = len(tab)//2
    if (w == tab[m]) :
        return True
    if (w<tab[m]) :
        return checkW(w, tab[0 : m])
    else:
        return checkW(w, tab[m+1 : len(tab)+1])


pathFew = 'words/english-few.txt'
with open(pathFew, 'r', encoding='utf-8') as file:
    content = file.read()

motsFew = [mot.strip() for mot in content.split(';') if mot.strip()]

pathAll = 'words/english-all.txt'
with open(pathAll, 'r', encoding='utf-8') as file:
    content = file.read()

motsAll = [mot.strip() for mot in content.split(';') if mot.strip()]

#print(motsAll)
#print(motsFew)

for mot in motsFew :
    if not checkW(mot, motsAll):
        print(mot)
