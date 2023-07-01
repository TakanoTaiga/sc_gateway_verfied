----------------------------- MODULE connection -----------------------------

EXTENDS TLC , Sequences

(*--algorithm connection
variables
    app_state = "preparing",
    in_transit_to_robot = <<>>,
    in_transit_to_app = <<>>;

fair process p_robot = "p_robot"
begin
    robot:
        if in_transit_to_robot = "searchNode" then
            \* in_transit_to_app := "ipResponse";
            in_transit_to_app := Append(in_transit_to_app, "ipResponse")
        elsif in_transit_to_robot = "pingRequest" then
            \* in_transit_to_app := "pingResponse";
            in_transit_to_app := Append(in_transit_to_app , "pingResponse")
        elsif in_transit_to_robot = "nodeInfoRequest" then
            \* in_transit_to_app := "nodeInfoResponse";
            in_transit_to_app := Append(in_transit_to_app , "nodeInfoResponse")
        end if;
end process;

fair process p_app_main = "p_app_main"
begin    
    app_lis_call:
        if Head(in_transit_to_app) = "ipResponse" then
            app_state := "ready";
        elsif Head(in_transit_to_app) = "pingResponse" then
             app_state := "ready";
        elsif Head(in_transit_to_app) = "nodeInfoResponse" then
            app_state := "ready";
        end if;

        in_transit_to_app := Tail(in_transit_to_app);
end process;

fair process p_app_timer = "p_app_timer"
begin    
    search_ros_node:
        if app_state /= "ready" then
            in_transit_to_robot := "searchNode";
        end if;
    node_info_collector:
        if app_state = "ready" then
            in_transit_to_robot := "nodeInfoRequest";
        end if;
    node_check_handler:
        if app_state = "ready" then
            app_state := "preparing";
            in_transit_to_robot := "pingRequest";
        end if;
end process;
            
end algorithm*)
\* BEGIN TRANSLATION (chksum(pcal) = "f336a7d1" /\ chksum(tla) = "eca0f4e9")
VARIABLES app_state, in_transit_to_robot, in_transit_to_app, pc

vars == << app_state, in_transit_to_robot, in_transit_to_app, pc >>

ProcSet == {"p_robot"} \cup {"p_app_main"} \cup {"p_app_timer"}

Init == (* Global variables *)
        /\ app_state = "preparing"
        /\ in_transit_to_robot = <<>>
        /\ in_transit_to_app = <<>>
        /\ pc = [self \in ProcSet |-> CASE self = "p_robot" -> "robot"
                                        [] self = "p_app_main" -> "app_lis_call"
                                        [] self = "p_app_timer" -> "search_ros_node"]

robot == /\ pc["p_robot"] = "robot"
         /\ IF in_transit_to_robot = "searchNode"
               THEN /\ in_transit_to_app' = Append(in_transit_to_app, "ipResponse")
               ELSE /\ IF in_transit_to_robot = "pingRequest"
                          THEN /\ in_transit_to_app' = Append(in_transit_to_app , "pingResponse")
                          ELSE /\ IF in_transit_to_robot = "nodeInfoRequest"
                                     THEN /\ in_transit_to_app' = Append(in_transit_to_app , "nodeInfoResponse")
                                     ELSE /\ TRUE
                                          /\ UNCHANGED in_transit_to_app
         /\ pc' = [pc EXCEPT !["p_robot"] = "Done"]
         /\ UNCHANGED << app_state, in_transit_to_robot >>

p_robot == robot

app_lis_call == /\ pc["p_app_main"] = "app_lis_call"
                /\ IF Head(in_transit_to_app) = "ipResponse"
                      THEN /\ app_state' = "ready"
                      ELSE /\ IF Head(in_transit_to_app) = "pingResponse"
                                 THEN /\ app_state' = "ready"
                                 ELSE /\ IF Head(in_transit_to_app) = "nodeInfoResponse"
                                            THEN /\ app_state' = "ready"
                                            ELSE /\ TRUE
                                                 /\ UNCHANGED app_state
                /\ in_transit_to_app' = Tail(in_transit_to_app)
                /\ pc' = [pc EXCEPT !["p_app_main"] = "Done"]
                /\ UNCHANGED in_transit_to_robot

p_app_main == app_lis_call

search_ros_node == /\ pc["p_app_timer"] = "search_ros_node"
                   /\ IF app_state /= "ready"
                         THEN /\ in_transit_to_robot' = "searchNode"
                         ELSE /\ TRUE
                              /\ UNCHANGED in_transit_to_robot
                   /\ pc' = [pc EXCEPT !["p_app_timer"] = "node_info_collector"]
                   /\ UNCHANGED << app_state, in_transit_to_app >>

node_info_collector == /\ pc["p_app_timer"] = "node_info_collector"
                       /\ IF app_state = "ready"
                             THEN /\ in_transit_to_robot' = "nodeInfoRequest"
                             ELSE /\ TRUE
                                  /\ UNCHANGED in_transit_to_robot
                       /\ pc' = [pc EXCEPT !["p_app_timer"] = "node_check_handler"]
                       /\ UNCHANGED << app_state, in_transit_to_app >>

node_check_handler == /\ pc["p_app_timer"] = "node_check_handler"
                      /\ IF app_state = "ready"
                            THEN /\ app_state' = "preparing"
                                 /\ in_transit_to_robot' = "pingRequest"
                            ELSE /\ TRUE
                                 /\ UNCHANGED << app_state, 
                                                 in_transit_to_robot >>
                      /\ pc' = [pc EXCEPT !["p_app_timer"] = "Done"]
                      /\ UNCHANGED in_transit_to_app

p_app_timer == search_ros_node \/ node_info_collector \/ node_check_handler

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == /\ \A self \in ProcSet: pc[self] = "Done"
               /\ UNCHANGED vars

Next == p_robot \/ p_app_main \/ p_app_timer
           \/ Terminating

Spec == /\ Init /\ [][Next]_vars
        /\ WF_vars(p_robot)
        /\ WF_vars(p_app_main)
        /\ WF_vars(p_app_timer)

Termination == <>(\A self \in ProcSet: pc[self] = "Done")

\* END TRANSLATION 
=============================================================================
\* Modification History
\* Last modified Wed Jun 28 21:52:21 JST 2023 by taigatakano
\* Created Wed Jun 28 21:50:33 JST 2023 by taigatakano
