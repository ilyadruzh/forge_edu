# Запуск

    cargo run

## Быстрая сортировка Хоара

на си: 
void qsort(int *ds, int *de, int *ss){
    int vl = *ds, *now = ds, *inl = ss, *ing = ss + (de - ds);
    if ( de <= ds + 1 ) return;
    for( ; now != de; ++now ){
        if ( *now <= vl ) *inl++ = *now;
        else *ing-- = *now;
    }
    *++inl = vl;
    qsort(ds, ds + (inl - ss), ss);
    qsort(ds + (inl - ss), de, inl + 1);
}

## Ряд тейлора

числа фибоначи