/* Student Schedule Optimization */
/* Author: Bartosz Michalak */
set Subjects;
set Groups;
set Days;

param Preferences{Groups, Subjects} >= 0;

param StartTime{Groups, Subjects} >= 0;
param EndTime{Groups, Subjects} >= 0;
param ClassDay{Groups, Subjects} symbolic;

var Chosen_Courses{Groups, Subjects}, binary;

maximize TotalPreference:
  sum{g in Groups, s in Subjects} Preferences[g,s] * Chosen_Courses[g,s];

# Constraint: One group per subject
s.t. OneGroupPerSubject{s in Subjects}:
    sum{g in Groups} Chosen_Courses[g,s] = 1;

s.t. NoOverlap{s1 in Subjects, g1 in Groups, s2 in Subjects, g2 in Groups:
    (s1 <> s2) and (
      ((StartTime[g1,s1] < StartTime[g2,s2]) and (EndTime[g2,s2] < EndTime[g1,s1])) or
      ((StartTime[g2,s2] < StartTime[g1,s1]) and (EndTime[g1,s1] < EndTime[g2,s2])) or
      ((StartTime[g1,s1] < EndTime[g2,s2]) and (EndTime[g2,s2] < EndTime[g1,s1])) or
      ((StartTime[g2,s2] < EndTime[g1,s1]) and (EndTime[g1,s1] < EndTime[g2,s2])))}:
    Chosen_Courses[g1,s1] + Chosen_Courses[g2,s2] <= 1;

# Constraint: No more than 4 hours per day
s.t. DailyHourLimit{d in Days}:
    sum{s in Subjects, g in Groups: (ClassDay[g,s] == d)}
        (EndTime[g,s] - StartTime[g,s]) * Chosen_Courses[g,s] <= 4;

# Constraint: Ensure lunch break (12:00-14:00 is free)
s.t. LunchBreak{d in Days}:
    sum{s in Subjects, g in Groups:
        (ClassDay[g,s] == d) and
        (StartTime[g,s] < 12 and EndTime[g,s] > 13 ) or # EndTime[g,s] - StartTime[g,s] >= 1 wynika z tego
        (StartTime[g,s] >= 12 and EndTime[g,s] <= 14 and EndTime[g,s] - StartTime[g,s] > 1) or
        (StartTime[g,s] < 13 and EndTime[g,s] > 14) # EndTime[g,s] - StartTime[g,s] >= 1 wynika z tego
    }
    Chosen_Courses[g,s] = 0;

# TODO warunki zle, np. zaczyna sie o 12 a konczy o 15w pon
# Constraint: At least one free training slot
s.t. Training:
    sum{s in Subjects, g in Groups:
        (EndTime[g,s] > 13 and StartTime[g,s] < 15 and ClassDay[g,s] == "Mon") or
        (EndTime[g,s] > 11 and StartTime[g,s] < 13 and ClassDay[g,s] == "Wed") or
        (EndTime[g,s] > 13 and StartTime[g,s] < 15 and ClassDay[g,s] == "Wed")
    } Chosen_Courses[g,s] <= 2;

solve;

display Chosen_Courses;
printf "Total preference: %.2f\n", TotalPreference;

end;
