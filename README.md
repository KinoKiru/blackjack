# blackjack
a blackjack game made in rust

How Blackjack works (i think):

Dealer lays down 2 cars for himself one is shown one is hidden,
then the players get 2 cards, the player desides if they want to either:
Hit and get another card,
Stand and stay with there cards.

If they hit and go over 21 they are bust and automaticly lose,
If they hit 21 and thus hit blackjack they win unless someone else hits blackjack in which case it is a draw.

If everyone has made there choice its the dealers turn, first he turns over the hidden card.
if his total value is 17 or above he can't hit and he can only stand then the winner is determined
if his total is below 17 he can hit untill he has crossed the 17 mark.
