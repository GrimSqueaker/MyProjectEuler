% numbers from 1 to 1000 in words
% -> letter count

a = 10*( ... % for all 0xx .. 9xx
         9*(3+3+5+4+4+3+5+5+4) ...    % one, two, .., nine for all but the teens
         + 10*(6+6+5+5+5+7+6+6) ...   % twenty, thirty, forty, fifty, sixty, seventy, eighty, ninety
         + 3+6+6+8+8 ...              % ten, eleven, twelve, thirteen, fourteen,
         + 7+7+9+8+8 ...              % fifteen, sixteen, seventeen, eighteen, nineteen
       ) ...
    + 100*(3+3+5+4+4+3+5+5+4+9*7) ... % one hundred, two hundred, ...
    + 9*99*3 ...                      % and
    + 3 + 8;                          % one thousand

a
