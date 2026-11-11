/***********************************************************************
                Module de Gestion de la Trois D v 2.0
                (C) God Le 4 D‚cembre 1986
***********************************************************************/
#include "sapiens.h"
#include <math.h>
#define VITESSE_DEPLACEMENT_OISEAUX 10
#define MONT1 1
#define MONT2 2
#define MONT3 3
#define MONT4 4
#define MONT5 5
#define TOTEM 6
#define NUAGE 7
#define FUMEE 18
#define FOYER 33
#define MOB_OISEAU 28
#define MOB_HOMME 35
#define ECLAIR 36
#define MOB_FOUDROYE 38
Forme
        forme1={ 5,0,0,900,
                 {800,660,150,100,1},
                 {9,9, 6,8, 0,0, 0,0, 0,0},BASEMOB+0,MONT1
               },
        forme2={ 8,2,0,1900,
                 {1900,1700,1100,1000,950,625,520,300},
                 { 0,10, 26,0, 0,1 ,0,0, 200,0, 200,0, 200,200 ,0,0},
                BASEMOB+1,MONT2
               },
        forme4={ 9,1 ,0,5000,
                {4100,3700,3250,2900,2500,2000,1500,1000,250},
                {240,240, 80,80, 0,1},BASEMOB+2,MONT4
               },

        forme6={ 14,1,0,10000,
                {9100,7400,6900,6490,5600,4000,3150,2500,2000,1500,1200,
                 850,550,200},
                {0,0 ,0,0 ,0,0},BASEMOB+3,MONT3
               },
        forme7={ 16,1 ,0,8000,
               {6500,4900,4100,3200,2700,2500,2300,2000,1500,1200,1100,1050,
                900,725,525,300},
                { 0,0, 0,0, 0,16, 16,16, 0,0},BASEMOB+4,MONT5
               },

        forme8={ 12,1 ,0,2000,
                {6000,4000,3100,2300,1900,1600,1400,1200,1100,900,500,100},
                { 0,0, 0,0, 0,16, 48,16, 0,0 ,0,0, 0,0, 0,0, 128,9, 128,8, 0,0,
                 0,0},
                BASEMOB+5,30
               },

        nuage1={ 8,3 ,6000,3500,
                {7500,6500,5500,7600,7000,3000,3700,20},
                {0,0, 192,192, 244,244, 0,0, 192,192, 244,244, 0,0, 192,192},-1,NUAGE
               },
        nuage2={ 9,3 ,4000,3500,
                {4000,5000,5500,5000,1000,4000,3000,500,3000},
                {-1,0, 192,192, 244,244, 0,0, 15,0, 244,244, 0,0, -1,-1, 0,0},-1,NUAGE+1
               },
        nuage3={ 5,3 ,3000,3500,
                {4000,3200,3000,1000,900},
                {-1,0, 192,192, 0,4, 0,208, 192,192},-1,NUAGE+2
               },
        eclair={ 10,3 ,100,500,
                {4000,3200,3000,1000,900},
                {-1,0, 192,192, 0,4, 0,208, 192,192},-1,ECLAIR
               },
        toucheEclair={ 10,4 ,2,60,
                {4,4,20,23,20,22,21,15,10,1},
                {0,0, 0,0, 0,0, 128,6, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0},
                BASEMOB+6,37
               },
        foudroye={ 10,4 ,1,60,
                {4,4,20,23,20,22,21,15,10,1},
                {0,0, 0,0, 0,0, 128,6, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0},
                BASEMOB+6,MOB_FOUDROYE
               },
        peupli={ 10,4 ,1,60,
                {4,4,20,23,20,22,21,15,10,1},
                {0,0, 0,0, 0,0, 128,6, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0},
                BASEMOB+6,10
               },
        chene={ 10,4 ,1,40,
                {10,6,10,40,45,47,40,35,26,5},
                {0,0, 0,0, 0,0, 0,0, 128,5, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,BASEMOB+6,11
          },
        noiset={ 10,4 ,1,40,
                {10,6,20,50,45,47,30,25,16,5},
                {0,0, 0,0, 0,0, 128,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,BASEMOB+6,12
           },
        fontaine={ 1,4 ,2,40,
                {10,6,20,50,45,47,30,25,16,5},
                {0,0, 0,0, 0,0, 128,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,-1,24
           },
        totem={ 10,4 ,2,40,
                {10,6,20,50,45,47,30,25,16,5},
                {0,0, 0,0, 0,0, 128,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,-1,TOTEM
           },
        hutte={ 1,4 ,2,40,
                {10,6,20,50,45,47,30,25,16,5},
                {0,0, 0,0, 0,0, 128,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,-1,32
           },
        oiseau={ 10,4 ,500,40,
                {10,6,20,50,45,47,30,25,16,5},
                {0,0, 0,0, 0,0, 128,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,-1,MOB_OISEAU
           },
        genevr={ 10,4 ,1,40,
                {4,4,15,13,11,9,8,6,4,2},
                {0,0, 0,0, 0,0, 128,5,  0,0, 0,0, 0,0, 0,0, 0,0, 0,0}
                        ,BASEMOB+6,13
                },
        hetre={ 10,4 ,1,40,
                {6,5,4,3,22,19,20,16,12,2},
                {0,0, 0,0, 0,0, 0,0, 0,0, 128,5, 0,0, 0,0, 0,0, 0,0}
                ,BASEMOB+6,14
                },
        roc1={ 8,1 ,0,210,
                {200,80,70,60,50,10,5,2},
                {0,0, 0,2, 1,3, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,BASEMOB+7,15
               },
        roc2={ 7,1 ,0,600,
                {500,400,225,190,160,120,10},
                {0,0, 2,2, 1,3, 0,10, 0,2, 0,0, 0,0, 0,0}
                ,BASEMOB+7,16
               },
        roc3={ 8,1 ,0,600,
                {3000,2500,2200,1800,800,750,650,600},
                {0,0, 2,10, -2,3, 0,10, 0,2, 0,0, 0,0, 0,0}
                ,BASEMOB+7,17
             },
        fumee={9,3,2,40,
                {2,10,20,70,100,160,100,200,20},
                {0,0, 2,2, 15,15, 15,15, 0,0, 15,15, 16,16, 32,32, 15,15}
                ,BASEMOB+8,FUMEE
             },

        foyer={ 1,4 ,2,20,
                {10,6,20,50,45,47,30,25,16,5},
                {0,0, 0,0, 0,0, 128,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,-1,FOYER
           },
        homme={ 1,4 ,1,20,
                {10,6,20,50,45,47,30,25,16,5},
                {0,0, 0,0, 0,0, 128,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,-1,35
           },

       building={ 12,1 ,0,100,
                {6000,4000,3100,2300,1900,1600,1400,1200,1100,900,500,100},
                { 0,0, 0,0, 0,16, 48,16, 0,0 ,0,0, 0,0, 0,0, 128,9, 128,8, 0,0,
                 0,0},
                BASEMOB+5,35
               },


        rocher={ 1,  /* Nombre de disques : 1 la hauteure sera finement regl‚e par l'objet */
                                 4 , /* Type de relief , inutlis‚ */
                                 1,  /* Hauteur 0= montagne,1=arbre */
                                 40, /* Rayon apprement : va conditionner l'aire horizontale */
                {10,6,20,50,45,47,30,25,16,5}, /* Disques : inutilis‚s */
                {0,0, 0,0, 0,0, 128,7, 0,0, 0,0, 0,0, 0,0, 0,0, 0,0}
                ,-1, /* Si -1 Pas de representation en carte */
                                34   /* Numero du mob dans Sapw3d */
           };

#define NB_OBJ_REF 223
Mont ObjetsRef [NB_OBJ_REF]= {
                        {00,00, 20000,1590,20,&forme1},
                        {00,00, 0x1400,0xD200,20,&forme2},
                        {00,00, 26000,16000,100,&forme4},
                        {00,00, 0x2700,0x41DC,140,&forme6},
                        {00,00, 0x8D25,0xD09A,200,&forme7},
                        {00,00, 0xC503,0x44E5,80 ,&forme8},
                                                /* Pieds agiles */
                        {00,00, 0x5880,0x5480,80,&fumee}, // 22672,21632
                        {00,00, 0x5880,0x5480,15,&foyer},
                        {00,00, 22672+32,21632,100,&hutte},
                        {00,00, 22672-32,21632,100,&hutte},
                                                /* Camps 2 */
                        {00,00, 0x9B00,0x3800,60,&fumee}, // 39680,14336
                        {00,00, 0x9B00,0x3800,15,&foyer},
                        {00,00, 39680+64,14336+64,140,&hutte},
                        {00,00, 39680+64+64,14336+64+32,100,&hutte},
                        {00,00, 39680+64+128,14336+64+32,100,&hutte},
                        {00,00, 39680+64+64,14336+256,100,&hutte},
                        {00,00, 39680-64-64,14336+256,100,&hutte},
                        {00,00, 39680-64-64,14336-256,100,&hutte},
                        {00,00, 39680-256,14336+64,100,&hutte},
                        {00,00, 39680-256,14336+512,100,&hutte},

                        {00,00, 0x2F00,0xCF00,60,&fumee},       // 12032,52992
                        {00,00, 0x2F00,0xCF00,15,&foyer},
                        {00,00, 12032-140,52992+90,100,&hutte},
                        {00,00, 12032-140+100,52992+90+80,100,&hutte},
                        {00,00, 12032-140-100,52992,100,&hutte},
                                                /* Camps 4 : hyenes folles */
                        {00,00, 0xC300,0xAA00,70,&fumee},       // 49920,43520
                        {00,00, 0xC300,0xAA00,15,&foyer},
                        {00,00, 49920+200,43520+200,100,&hutte},
                        {00,00, 49920+200+100,43520+200,100,&hutte},
                        {00,00, 49920+200+100,43520+200+100,100,&hutte},
                        {00,00, 49920+32,43520,15,&totem},

                        {00,00, 4000,4000  ,20,&forme2},
                        {00,00, 9000,5000,33,&forme2},
                        {00,00, 9000,5000,30,&forme2},
                        {00,00,13000,3500,16 ,&forme2},
                        {00,00,8000 ,1800,26 ,&forme2},
                        {00,00,40000L,5000,33,&forme2},
                        {00,00,25000,52000L,30,&forme2},
                        {00,00, 9000,5000,30,&forme2},
                        {00,00, 37000L,30000,16,&forme2},
                        {00,00, 32000,32000,33,&forme2},
                        {00,00, 43000L,42000L,25,&forme2},
                        {00,00, 4000,50000L,46,&forme2},
                        {00,00, 33000L,8000,120,&forme4},
                        {00,00, 55000L,5000,100,&forme4},
                        {00,00, 38000L,23500,140,&forme7},
                        {00,00, 28000,26000,100,&forme4},
                        {00,00, 13000,37000L,120,&forme7},
                        {00,00, 37000L,37000L,100,&forme4},
                        {00,00, 47000L,35000L,100,&forme4},
                        {00,00, 33000L,8000,120,&forme4},
                        {00,00, 55000L,42000L,120,&forme4},
                        {00,00, 5000,32000,140,&forme4},
                        {00,00, 20000,30000,120,&forme4},
                        {00,00, 25000,40000L,120,&forme7},
                        {00,00, 0x5880,0x5320,8,&chene},
                        {00,00, 0x5700,0x5200,8,&genevr},
                        {00,00, 0x5900,0x5200,8,&peupli},
                        {00,00, 0xABE0,0xB2E8,20,&totem},
                        {00,00, 0xABE0,0xB34C,20,&totem},
                        {00,00, 0xAB7C,0xB2E8,20,&totem},
                        {00,00, 25000, 22500,10,&chene},
                        {00,00, 19000, 24000,8,&chene},
                        {00,00, 13000, 42500L,10,&chene},
                        {00,00, 3000, 30000,12,&chene},
                        {00,00, 4500, 46000L,4,&chene},
                        {00,00, 17000, 44000L,15,&chene},
                        {00,00, 17500, 49000L,9,&chene},
                        {00,00, 32000, 45000L,10,&chene},
                        {00,00, 19200, 47200L,12,&chene},
                        {00,00, 9000, 45000L,7,&chene},
                        {00,00, 32100, 15050,15,&chene},
                        {00,00, 32200, 15200,12,&chene},
                        {00,00, 32300, 15400,10,&chene},
                        {00,00, 32400, 15410,10,&chene},
                        {00,00, 32000, 15600,10,&chene},
                        {00,00, 32050, 15700,7,&chene},
                        {00,00, 12000, 50000L,7,&chene},
                        {00,00, 32000, 15800,10,&chene},
                        {00,00, 31900, 15900,15,&chene},
                        {00,00, 32050, 15700,7,&chene},
                        {00,00, 33000L, 16000,17,&chene},
                        {00,00, 33500L, 16000,10,&chene},
                        {00,00, 31900, 15900,15,&chene},
                        {00,00, 33000L, 13500,10,&chene},
                        {00,00, 38000L, 12000,7,&chene},
                        {00,00, 32000, 15000,7,&chene},
                        {00,00, 32800L, 18100,10,&chene},
                        {00,00, 33050L, 16500,9,&chene},
                        {00,00, 32800L, 18200,10,&chene},
                        {00,00, 35000L, 16000,7,&chene},
                        {00,00, 25000, 21500,10,&chene},
                        {00,00, 10900, 28000,12,&chene},
                        {00,00, 13000, 31000,10,&chene},
                        {00,00, 15000, 26000,10,&chene},
                        {00,00, 29000, 20500,9,&chene},
                        {00,00, 21700, 8500,12,&chene},
                        {00,00, 23800, 6000,9,&chene},
                        {00,00, 20000, 5500,9,&chene},
                        {00,00, 24000, 5000,9,&chene},
                        {00,00, 26000, 10000,9,&chene},
                        {00,00, 20000, 9000,9,&chene},
                        {00,00, 15000, 6000,9,&chene},
                        {00,00, 15500,47000L,15,&peupli},
                        {00,00, 24000,48500L,20,&peupli},
                        {00,00, 20500,18000,12,&peupli},
                        {00,00, 40000L,200,18,&peupli},
                        {00,00, 34000L,2000,16,&peupli},
                        {00,00, 32000,1000,12,&peupli},
                        {00,00, 100,41000L,12,&peupli},
                        {00,00, 22000,25000,18,&peupli},
                        {00,00, 21000,21000,20,&peupli},
                        {00,00, 13500,44000L,12,&peupli},
                        {00,00, 33500L,19000,12,&peupli},
                        {00,00, 42000L,17000,20,&peupli},
                        {00,00, 34000L,17000,12,&peupli},
                        {00,00, 38000L,16000,9,&peupli},
                        {00,00, 39050L,14050,12,&peupli},
                        {00,00, 21000,23000,14,&peupli},
                        {00,00, 12100,29050,12,&peupli},
                        {00,00, 11000,30000,20,&peupli},
                        {00,00, 13000,46000L,10,&hetre},
                        {00,00, 17500,45000L,15,&hetre},
                        {00,00, 19000,47000L,10,&hetre},
                        {00,00, 19000,48500L,18,&hetre},
                        {00,00, 22000,23000,10,&hetre},
                        {00,00, 13000,42500L,15,&hetre},
                        {00,00, 3500,47000L,10,&hetre},
                        {00,00, 7000,41000L,11,&hetre},
                        {00,00, 32900L,17000,15,&hetre},
                        {00,00, 40000L,14000,11,&hetre},
                        {00,00, 40000L,15000,8,&hetre},
                        {00,00, 39000L,12200,11,&hetre},
                        {00,00, 32800L,18000,15,&hetre},
                        {00,00, 42000L,15000,18,&hetre},
                        {00,00, 41500L,14000,15,&hetre},
                        {00,00, 42500L,16000,15,&hetre},
                        {00,00, 31500,22000,15,&hetre},
                        {00,00, 12000,29000,17,&hetre},
                        {00,00, 11900,29100,15,&hetre},
                        {00,00, 12500,30000,15,&hetre},
                        {00,00, 5000,39000L,10,&genevr},
                        {00,00, 19250,49200L,15,&genevr},
                        {00,00, 35000L,43000L,4,&genevr},
                        {00,00, 41000L,2000,2,&genevr},
                        {00,00, 4000,40000L,6,&genevr},
                        {00,00, 14000,45000L,2,&genevr},
                        {00,00, 14000,45000L,12,&genevr},
                        {00,00, 19100,47050L,10,&genevr},
                        {00,00, 31500,14500,10,&genevr},
                        {00,00, 34000L,15000,10,&genevr},
                        {00,00, 36000L,17000,10,&genevr},
                        {00,00, 40000L,18000,10,&genevr},
                        {00,00, 31000,19000,12,&genevr},
                        {00,00, 14000,30000,10,&genevr},
                        {00,00, 10000,29500,10,&genevr},
                        {00,00, 13000,27000,10,&genevr},
                        {00,00, 40000L,42000L,10,&genevr},
                        {00,00, 41000L,44000L,10,&genevr},
                        {00,00, 47000L,45000L,10,&genevr},
                        {00,00, 48000L,44000L,10,&genevr},
                        {00,00, 48000L,48000L,10,&genevr},
                        {00,00, 21000,20000,10,&noiset},
                        {00,00, 21000,20000,10,&fontaine},
                        {00,00, 12400,28500,10,&noiset},
                        {00,00, 12400,28500,10,&fontaine},
                        {00,00, 48000L,42000L,10,&noiset},
                        {00,00, 48000L,42000L,10,&fontaine},
                        {00,00, 2000,2000,10,&noiset},
                        {00,00, 2000,2000,10,&fontaine},
                        {00,00, 39000L,800,10,&noiset},
                        {00,00, 39000L,800,10,&fontaine},
                        {00,00, 16000,42000L,10,&noiset},
                        {00,00, 16000,42000L,10,&fontaine},
                        {00,00, 32000,16000,10,&noiset},
                        {00,00, 32000,16000,10,&fontaine},
                        {00,00, 45000L,25500,10,&noiset},
                        {00,00, 45000L,25500,10,&fontaine},
                        {00,00, 6000,45000L,10,&noiset},
                        {00,00, 6000,45000L,10,&fontaine},
                        {00,00, 50000L,50000L,10,&noiset},
                        {00,00, 50000L,50000L,5,&fontaine},
                        {00,00, 24320L,18944L,10,&noiset},/* Nouvelles sources */
                        {00,00, 24320L,18944L,10,&fontaine},
                        {00,00, 32768L,17408L,10,&noiset},
                        {00,00, 32768L,17408L,5,&fontaine},
                        {00,00, 40192L,14336L,10,&noiset},
                        {00,00, 40192L,14336L,5,&fontaine},
                        {00,00, 43776L,16896L,10,&noiset},
                        {00,00, 43776L,16896L,5,&fontaine},

                        {00,00, 100,39000L,100,&roc1},
                        {00,00, 15000,46000L,45,&roc2},
                        {00,00, 42000L,16000,75,&roc3},
                        {00,00, 33050L,17000,95,&roc1},
                        {00,00, 5000,38000L,100,&roc2},
                        {00,00, 7000,39000L,150,&roc3},
                        {00,00, 4000,43000L,110,&roc1},
                        {00,00, 7000,44000L,100,&roc2},
                        {00,00, 10000,42000L,150,&roc1},
                        {00,00, 15000,43000L,100,&roc2},
                        {00,00, 15000,45000L,140,&roc1},
                        {00,00, 18000,47000L,98,&roc2},
                        {00,00, 19000,50000L,100,&roc1},
                        {00,00, 42000L,46000L,100,&roc1},
                        {00,00, 44000L,45000L,200,&roc2},
                        {00,00, 46000L,42000L,100,&roc2},
                        {00,00, 38000L,44000L,90,&roc1},
                        {00,00, 35000L,1000,110,&roc2},
                        {00,00, 37000L,3000,100,&roc2},
                        {00,00, 36000L,15000,110,&roc2},
                        {00,00, 37000L,3000,100,&roc2},
                        {00,00, 39000L,13000,100,&roc2},
                        {00,00, 41000L,13000,100,&roc2},
                        {00,00, 36000L,18000,100,&roc2},
                        {00,00, 39000L,17000,100,&roc2},
                        {00,00, 38000L,12000,132,&roc2},
                        {00,00, 42000L,15000,100,&roc2},
                        {00,00, 20000,22000,100,&roc2},
                        {00,00, 13000,28000,100,&roc2},
                        {00,00, 11000,32000,100,&roc2},
                        {00,00, 46000L,47500L,100,&roc2},
                        {00,00, 9500,50000L,100,&roc3},
                        {00,00, 0001,0001,10,&chene}
                        };
/* Liste des objets pr‚sents */
MontHandle aigoual;
long nbreobj;
static short PremHomme;
long nbNuages,nbOiseaux,nbRochers,nbHommes,nbArbres,FreqOrage,DistMinFoudroie,VitesseDeplacementNuages;
static long PremEclair,PremArbre,PremRocher;
static MontPtr ArbreFoudroye;
long nbEclairs;
long FreqEclair;
/***********************************************************************
                         AjouteObjet3d
Ajoute un objet … la liste des objets 3ds
=> rien
<= index
***********************************************************************/
long AjouteObjet3d(long nbObj)
{
OSErr err;

if(aigoual==NULL) aigoual=(MontHandle)NewHandle(0L);
nbreobj+=nbObj;
SetHandleSize((Handle)aigoual,(long)nbreobj*(long)sizeof(Mont));
err=MemError();
if(err)
        Collapse("[Sap3d.AjouteObjet3d.1] Error # %d",err);
return(nbreobj-nbObj);
}
/***********************************************************************
                         EstDansMontagne
Dit si une position est dans une montagne
***********************************************************************/
static Boolean EstDansMontagne(unsigned short  x,unsigned short  y)
{
long i;
long dx,dy,dist;

for(i=0;i<nbreobj;i++)
        {
        if((*aigoual)[i].ptrforme && (*aigoual)[i].ptrforme->hauteur==0)
                {
                dx=abs((*aigoual)[i].posx-x);
                dy=abs((*aigoual)[i].posy-y);
                dist=distance(dx,dy);
                if(dist<(*aigoual)[i].ptrforme->rayon) return(VRAI);
                }
        }
return(FAUX);
}
/***********************************************************************
                         DeplaceHommes
***********************************************************************/
typedef struct
                {
                unsigned short top,left,right,bottom;
                } GrandRect;

typedef struct
                {
                short existe;
                short x,y;
                short vx,vy;
                GrandRect rect;
                } Homme,*HommePtr;
static HommePtr TabHomme;
void DeplaceHommes(void)
{
HommePtr h=TabHomme;
MontPtr mont=&((*aigoual)[PremHomme]);
short i;

for(i=0;i<nbHommes;i++,h++,mont++)
        {
        /* L'objet n'est pas defini ?*/
        if(h->existe==FAUX)
                {
                (*aigoual)[PremHomme+i].ign=myRandom(50,60);// Actif
                h->existe=VRAI;
                h->vx=myRandom(-5,5);
                h->vy=myRandom(-5,5);
                switch(myRandom(0,4))
                        {
                        default:
                                h->rect.left=22672-1000;
                                h->rect.right=22672+1000;
                                h->rect.top=21632-1000;
                                h->rect.bottom=21632+1000;
                                h->x=22672;
                                h->y=21632;
                                break;
                        }

                }
        else
                {
                /* L'objet se deplace */
                h->x+=h->vx;
                h->y+=h->vy;
                if(h->x<h->rect.left || h->x>h->rect.right) h->vx=-h->vx;
                if(h->y<h->rect.top || h->y>h->rect.bottom) h->vy=-h->vy;
                mont->posx=h->x;
                mont->posy=h->y;
                if(mont->distance<200)
                                {
                                h->existe=FAUX;
                                mont->ign=0;// InActif
                                }
                }
        }
}
/***********************************************************************
                         GenereHommes
***********************************************************************/
void GenereHommes(void)
{
long i,index;

PremHomme=-1;
TabHomme=(HommePtr)NewPtrClear((long)sizeof(Homme)*nbHommes);
for(i=0;i<nbHommes;i++)
        {
        index=AjouteObjet3d(1);
        if(PremHomme<0) PremHomme=index;
        (*aigoual)[index].ign=0; // Non visible
        (*aigoual)[index].ptrforme=&homme;
        }
}

/***********************************************************************
                         DefautDecor3d
3D au demarrage du jeu
**********************************************************************/
void DefautDecor3d(void)
{
FreqOrage=FREQ_ORAGE;//9990
DistMinFoudroie=DIST_MIN_FOUDROIE;//250
nbNuages=NB_NUAGES;//100
nbOiseaux=NB_OISEAUX;//20
nbRochers=NB_ROCHERS;//300
nbHommes=NB_HOMMES;//0
nbArbres=NB_ARBRES; //800
nbHerbes=NB_HERBE_AV_PLAN;
FreqEclair=1000;//1000
VitesseDeplacementNuages=20;
return;
}
/***********************************************************************
                         InitDecor3d
Tire au hasard les ferquences a chaque nlle aventure
***********************************************************************/
void InitDecor3d(void)
{
FreqOrage=myRandom(9900,10000);
DistMinFoudroie=myRandom(20,500);
nbNuages=myRandom(NB_NUAGES,NB_NUAGES_MAX);
nbOiseaux=myRandom(NB_OISEAUX,NB_OISEAUX_MAX);
nbRochers=myRandom(NB_ROCHERS,NB_ROCHERS_MAX);
nbArbres=myRandom(NB_ARBRES,NB_ARBRES_MAX); //800
nbHerbes=myRandom(NB_HERBE_AV_PLAN,NB_MAX_HERBE_AV_PLAN);
nbHommes=0;//0
FreqEclair=1000;
VitesseDeplacementNuages=(long)myRandom(1,120);
}
/***********************************************************************
                         CreeDecor3d
Cree la liste des objets 3ds
***********************************************************************/
void CreeDecor3d(Boolean display)
{
long i,index;
Mont mont;

if(display) 
	{
	flefba=0;
	comment(137);//Je cree le monde
	ValideUpdate();
	}
initrnd('M','Y','R');

#if DEBUG>0
	printf("FreqOrage %ld\n",FreqOrage);
	printf("DistMinFoudroie %ld\n",DistMinFoudroie);
	printf("nbNuages %ld\n",nbNuages);
	printf("nbOiseaux %ld\n",nbOiseaux);
	printf("nbRochers %ld\n",nbRochers);
	printf("nbArbres %ld\n",nbArbres);
#endif
nbEclairs=nbNuages;
if(aigoual) DisposHandle((Handle)aigoual);
aigoual=NULL;
PremEclair=0;
PremArbre=0;
PremRocher=0;
ArbreFoudroye=NULL;
nbreobj=0;
for(i=0;i<nbNuages;i++)
	{
	mont.posx=(long)myRandom(0,32767)*2L;
	mont.posy=(long)myRandom(0,32767)*2L;
	mont.ign=myRandom(100,200);
	switch(myRandom(0,3))
		{
		case 0:mont.ptrforme=&nuage1;break;
		case 1:mont.ptrforme=&nuage2;break;
		default:mont.ptrforme=&nuage3;break;
		}
	index=AjouteObjet3d(1);
	(*aigoual)[index]=mont;
	}
/* A chaque nuage un eclair est associe */
for(i=0;i<nbEclairs;i++)
	{
	mont=(*aigoual)[i];//Copie du nuage associe
	mont.ign=0; // Altitude zero inactif
	mont.ptrforme=&eclair;
	mont.posx+=(*aigoual)[i].ptrforme->rayon/2;
	mont.posy+=(*aigoual)[i].ptrforme->rayon/2;
	index=AjouteObjet3d(1);
	(*aigoual)[index]=mont;
	if(PremEclair==0) PremEclair=index;
	}
for(i=0;i<nbOiseaux;i++)
	{
	mont.posx=(long)myRandom(0,32767)*2L;
	mont.posy=(long)myRandom(0,32767)*2L;
	mont.ign=10;
	mont.ptrforme=&oiseau;
	index=AjouteObjet3d(1);
	(*aigoual)[index]=mont;
	}
index=AjouteObjet3d(NB_OBJ_REF);
/* On cree le tableau avec les objets de base */
for(i=0;i<NB_OBJ_REF;i++)
        (*aigoual)[index+i]=ObjetsRef[i];
// les arbres
for(i=0;i<nbArbres;i++)
	{
	do
		{
		mont.posx=(long)myRandom(0,32767)*2L;
		mont.posy=(long)myRandom(0,32767)*2L;
		} while(EstDansMontagne(mont.posx,mont.posy));
	mont.ign=myRandom(5,20);
	switch(myRandom(0,3))
		{
		case 0:mont.ptrforme=&genevr;break;
		case 1:mont.ptrforme=&chene;break;
		default:mont.ptrforme=&peupli;break;
		}
	index=AjouteObjet3d(1);
	if(PremArbre==0) PremArbre=index;
	(*aigoual)[index]=mont;
	}
// Les rochers
for(i=0;i<nbRochers;i++)
	{
	do
		{
		mont.posx=(long)myRandom(0,32767)*2L;
		mont.posy=(long)myRandom(0,32767)*2L;
		} while(EstDansMontagne(mont.posx,mont.posy));
	mont.ign=myRandom(15,60);
	mont.ptrforme=&rocher;
	index=AjouteObjet3d(1);
	if(PremRocher==0) PremRocher=index;
	(*aigoual)[index]=mont;
	}
#if DEBUG>0
		printf("nbreobj %ld\n",nbreobj);

#endif
//GenereHommes();
return;
}

/***********************************************************************
				WaitTick
=> nbre de tickcount a attendre
***********************************************************************/
static void WaitTick(short nb)
{
long t=TickCount();

while(TickCount()<t+nb) /**/;
}

/***********************************************************************
                         Final
Gestion de l'animation finale
***********************************************************************/
void Final(void)
{
long i;
short t;
long n;
long dx,dy,dist;
long index;
Mont mont;
RGBColor rgb;
char chaine[255];

SetCursor(*GetCursor(watchCursor));
DefautDecor3d();
heure=0;
while(heure<40) chrono(1);
jour_dsk=0;
xj=0x5800 ;
yj=0x5400 ;
dzeta=768;
 zj=40;
clrbadi();
comment(133);/* Le temps passe */
for(n=0;n<5;n++)
    {
    /* Les arbres augmentent en nombre */
    for(t=0;t<15;t++)
        {
        for(i=0;i<nbNuages;i++)
            {
            (*aigoual)[i].posx=xj+myRandom(-5000,5000);
            (*aigoual)[i].posy=yj+myRandom(-5000,5000);
            }
        GereInterface();
        for(i=PremArbre;i<PremArbre+nbArbres;i++)
                {
                 dx=abs((*aigoual)[i].posx-xj);
                 dy=abs((*aigoual)[i].posy-yj);
                 dist=distance(dx,dy);
                 /* L'arbre est eloigne de plus de 2000 m */
                 if(dist>2000 && myRandom(0,1000)>900)
                    {
                    /* On le rapproche */
                    (*aigoual)[i].posx=xj+myRandom(-750,750);
                    (*aigoual)[i].posy=yj+myRandom(-5000,-500);
                    dx=abs((*aigoual)[i].posx-xj);
                    dy=abs((*aigoual)[i].posy-yj);
                    dist=distance(dx,dy);
                    if(dist<50)
                        {
                        (*aigoual)[i].posx=65000;
                        (*aigoual)[i].posy=65000;
                        }
                    }
                }
        AvanceSol(1);
        heure=myRandom(0,90);
        affich(dzeta);
        tfrima();
        delai(0);
        WaitTick(5);
        }
    /* Erosion des montagnes */
    for(i=0;i<nbreobj;i++)
        {
        if((*aigoual)[i].ptrforme && (*aigoual)[i].ptrforme->hauteur==0)
            {
            /* C'est une montagne elle diminue*/
            if((*aigoual)[i].ign>10) (*aigoual)[i].ign-=10;
            }
        }
    /* Erosion des montagnes */
    for(i=PremRocher;i<PremRocher+nbRochers;i++)
        {
         dx=abs((*aigoual)[i].posx-xj);
         dy=abs((*aigoual)[i].posy-yj);
         dist=distance(dx,dy);
         if(dist>2000 && myRandom(0,1000)>900)
                    {
                    (*aigoual)[i].posx=xj+myRandom(-1000,1000);
                    (*aigoual)[i].posy=yj+myRandom(-1000,-100);
                    dx=abs((*aigoual)[i].posx-xj);
                    dy=abs((*aigoual)[i].posy-yj);
                    dist=distance(dx,dy);
                    if(dist<50)
                        {
                        (*aigoual)[i].posx=65000;
                        (*aigoual)[i].posy=65000;
                        }
                    }
        }
    /* Les arbres diminuent en nombre */
    for(t=0;t<15;t++)
        {
        for(i=0;i<nbNuages;i++)
            {
            (*aigoual)[i].posx=xj+myRandom(-5000,5000);
            (*aigoual)[i].posy=yj+myRandom(-5000,5000);
            }
        GereInterface();
        for(i=PremArbre;i<PremArbre+nbArbres;i++)
                {
                 dx=abs((*aigoual)[i].posx-xj);
                 dy=abs((*aigoual)[i].posy-yj);
                 dist=distance(dx,dy);
                 if(dist<3000 && myRandom(0,1000)>900)
                    {
                    (*aigoual)[i].posx=65000;
                    (*aigoual)[i].posy=65000;
                    }
                }
        heure=myRandom(0,90);
        AvanceSol(1);
        affich(dzeta);
        tfrima();
        delai(0);
        WaitTick(5);
        }
    }
for(i=0;i<nbNuages;i++)
            {
            (*aigoual)[i].posx=xj+myRandom(-2000,2000);
            (*aigoual)[i].posy=yj+myRandom(-2000,2000);
            }
for(n=0;n<100;n++)
    {
    FreqEclair=900;
        /* Erosion des rochers */
    for(i=PremRocher;i<PremRocher+nbRochers;i++)
        {
         if( myRandom(0,1000)>900)
           {
           (*aigoual)[i].posx=65000;
           (*aigoual)[i].posy=65000;
           }
        }
    /* Disparition des arbres */
    for(i=PremArbre;i<PremArbre+nbArbres;i++)
        {
         if( myRandom(0,1000)>900)
           {
           (*aigoual)[i].posx=65000;
           (*aigoual)[i].posy=65000;
           }
        }
    mont.ign=myRandom(15,60);
    mont.ptrforme=&building;
    mont.posx=xj+myRandom(-750,750);
    mont.posy=yj+myRandom(-10000,-500);
    index=AjouteObjet3d(1);
    (*aigoual)[index]=mont;
    heure=myRandom(0,90);
    affich(dzeta);
    tfrima();
    delai(0);
    chrono(1);
    WaitTick(5);
    }
heure=512;
for(n=0;n<400;n++)
    {
    yj-=30;
    affich(dzeta);
    tfrima();
    delai(0);
    chrono(1);
    WaitTick(2);
    }
AcamSetPortOffscreen(Ecran1);
TextFont(times);
DoTextSize(64);
strcpy(chaine,"THE END");
CtoPstr(chaine);
dx=(512-StringWidth((StringPtr)chaine))/2L;
for(n=0;n<65535;n+=256)
    {
    chrono(1);
    chrono(1);
    rgb.red=rgb.green=rgb.blue=n;
    RGBForeColor(&rgb);
    DoMoveTo(dx,64);
    affich(dzeta);
    delai(0);
    DrawString((StringPtr)chaine);
    tfrima();
     WaitTick(1);
    }
CarboniseSetCursorArrow();
CarboniseSetPortWindow(mainWindow);
return;
}
/***********************************************************************
                        FoudroieJoueur
Applique un eclair sur le plan a la postion donn‚e
**********************************************************************/
static void FoudroieJoueur(unsigned short x,unsigned short y)
{
short i;

/* Le joeur est foudroy‚ par l'eclair lateral*/
if(distance(xj-x,yj-y)<DistMinFoudroie)
    {
    clrbadi();
    comment(134);
    for(i=0;i<100;i++)
                {
                effima(0x077);
                tfrima();
                effima(0x000);
                tfrima();
                }
    /* Le joueur est mort */
    mort=0;
    }
}
/***********************************************************************
                        FoudroieArbre
Applique un eclair sur le plan a la postion donn‚e
**********************************************************************/
static MontPtr FoudroieArbre(unsigned short x,unsigned short y)
{
MontPtr mont;
long i;

FoudroieJoueur(x,y);
mont=(*aigoual);
i=nbreobj;
while(i--)
    {
    /* C'est un arbre et meme coordonnee */
    if(mont->ptrforme->hauteur==1 && distance(mont->posx-x,mont->posy-y)<DistMinFoudroie)
        {
       	mont->ptrforme=&toucheEclair;
        FoudroieJoueur(mont->posx,mont->posy);
        return(mont);
        }
    mont++;
    }
return(NULL);
}
/***********************************************************************
                       ChargeEltsDecorsMobiles
**********************************************************************/
void ChargeEltsDecorsMobiles(FILE * f)
{
char chaine[255];
long x,y;
MontPtr mont;
long i;

fscanf(f," %s",chaine);
if(strcmp(chaine,"MAGIC")) return;
while(!feof(f))
    {
    chaine[0]=0;
    fscanf(f,"%s",chaine);
    if(!strcmp(chaine,"Foudroye"))
        {
        fscanf(f," %ld %ld",&x,&y);
        i=nbreobj;
        mont=(*aigoual);
        while(i--)
            {
            if(mont->ptrforme->hauteur==1 && mont->posx==x && mont->posy==y)
                mont->ptrforme=&foudroye;
            mont++;
            }
        }

    }
}
/***********************************************************************
                       SauveEltsDecorsMobiles
Sauve dans le fichier aventure en entr‚e les elements varaibles du d‚cor:
arbres foudroy‚s...
**********************************************************************/
void SauveEltsDecorsMobiles(FILE * f)
{
MontPtr mont;
long i=nbreobj;

mont=(*aigoual);
fprintf(f,"\nMAGIC\n");
while(i--)
    {
    /* C'est un arbre et meme coordonnee */
    if(mont->ptrforme==&foudroye)
        {
        fprintf(f,"Foudroye %ld %ld\n",(long)mont->posx,(long)mont->posy);
        }
    mont++;
    }
return;
}
/***********************************************************************
                        AFFICH:Affiche 3 d
L'affichage se fait sur l'ecran1
**********************************************************************/
void affich(unsigned short tfr)
{
long i;
short dzetabak;
MontPtr suivant;
dzetabak=dzeta;
dzeta=tfr;

/* On efface le fond */
eff3d(ciel,terre,metpos,tfr+128-angsol,(heure & 0x200)== 0x200);
/*mets fum‚e en place*/
fumee.numMob3d=myRandom(18,20);
if(++forme8.numMob3d==32)forme8.numMob3d=30;
if(++oiseau.numMob3d==MOB_OISEAU+2) oiseau.numMob3d=MOB_OISEAU;
if(++homme.numMob3d==MOB_HOMME+2) homme.numMob3d=MOB_HOMME;
/*deplace nuages vers l'ouest*/
for (i=0;i<nbNuages;i++) (*aigoual)[i].posx+=VitesseDeplacementNuages;
// Delai avant le premier eclair
if(FreqEclair>1000)
    {
    /* Pas d'orage en cours, on en genere un ? */
    if(myRandom(0,10000)>FreqOrage)
        FreqEclair=0;
    }
else
    FreqEclair+=4;
if(ArbreFoudroye)
    {
    ArbreFoudroye->ptrforme=&foudroye;
    ArbreFoudroye=NULL;
    }
suivant=&((*aigoual)[PremEclair]);
for (i=0;i<nbEclairs;i++,suivant++)
    {
    /* Fin de l'eclair */
    if(suivant->ign)
        {
        if(ArbreFoudroye==NULL)
           ArbreFoudroye=FoudroieArbre(suivant->posx,suivant->posy);
        suivant->ign=0;
		#if DEBUG>0
		if(suivant->ptrforme->numMob3d==7)
				Collapse("err");
        #endif
		}
    else if(myRandom(0,1000)>FreqEclair && myRandom(0,1000)>910) 
		suivant->ign=300;//Demarre eclair
    /* on deplace toujours l'eclair de facon a ce qu'il soit cole au nuage */
    suivant->posx+=VitesseDeplacementNuages;
    }
/* Deplace oiseaux vers le sud */
for (i=0;i<nbOiseaux;i++) (*aigoual)[nbNuages+i].posy+=VITESSE_DEPLACEMENT_OISEAUX;

/*on trie les montagnes , les nuages ,et les arbres*/
(*aigoual)[0].chaine=0;/*depart de la chaine sur 1 er elt */
for (i=1;i<nbreobj;i++) ChaineObjet(&(*aigoual)[i]);
/* Deplacement des objets vivants */
//DeplaceHommes();
/*on affiche*/
suivant=(*aigoual)[0].chaine;
/*on envoit la position en memoire de l'elt*/
if (suivant)
                visu(suivant);
/* On traite la pluie */
Pluie();
dzeta=dzetabak;
return;
}
/***********************************************************************
                        CLAV3D Routine maitre durant la 3d
***********************************************************************/
static short okcarte=0;
void clav3d(void)
{
char car,cptemps=1,cpt_cart=0;
Boolean cont=VRAI;
unsigned long posTimer;

dismin=0;
//reset_mouse();
/* On ne remets a zero que la position en colonne */
do vq_mouse(&bouton,&dsposx,&sposy);
        while(bouton);

do
    {
    posTimer=sapiensTimer;
    //bouge_souris();
    affich(dzeta);
    tfrima();
    if (okcarte)
        {
        if (!(cpt_cart--))
                {
                cpt_cart=5;
                carte();
                }
        }
    sapdep(dzeta,sapvit);
    if (! (--cptemps))
                    {
                    cptemps=5;
                    chrono(0);
                    }
    else
                    chrono(1);
    zoninf();
    PosePluie();
    car=getkey3d();
    if (car)
            {
            if ((car & 1) && sapvit<20)
                                        {
                                        icone=mobBambou+23;
                                        sapvit+=2;
                                        }
            if ((car & 2) && sapvit )
                                        {
                                        icone=mobBambou+24;
                                        if(sapvit<4) sapvit=0;
                                        else sapvit -=4;
                                        }
            if (car & 8)
                                        {
                                        icone=mobBambou+26;
                                        dzeta=(dzeta-8) & 0x3FF;
                                        }
            if (car & 4)
                                        {
                                        icone=mobBambou+25;
                                        dzeta=(dzeta+8) & 0x3FF;
                                        }
            /* si bouton de gauche */
            if (car & 32)
                    {
                    while(getkey3d()&32)/**/;/* Anti repeat */
                    okcarte ^=1;
                    if (okcarte)  cpt_cart=0;
                    else    affomb(DEBFLECH+17,180,280,5,5,VRAI);/* OFF */
                    }
            /* Sortie */
            if (car & 64)   cont=FAUX;
            }
        else
                        icone=mobBambou+20;
	/* Synchronisation de la trame */                        
	while (posTimer > sapiensTimer - 9) {}

    } while(!presence && mort && cont);
/* On transfere l'ecran1 sur debecr */
tfrvirt();
/* Un presence humaine ? */
if(presence<=4 && presence)
    {
     /* On ouvre par defaut le menu'dire'*/
    InverseOption(0,4);
    s_action=5;
    inimen(2);
    }
return;
}
/*---- fin de texte ----*/






