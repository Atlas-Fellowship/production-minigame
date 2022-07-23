import { DisplayModal } from "@innexgo/common-react-components";
import { ApiKey } from "@innexgo/frontend-auth-api";
import React from "react";
import Button from "react-bootstrap/esm/Button";
import Card from "react-bootstrap/esm/Card";
import { TournamentData, TournamentSubmission, TournamentYearDemand } from "../utils/api";
import CreateTournamentSubmission from "./CreateTournamentSubmission";

type CompeteTournamentManagerProps = {
  tournamentData: TournamentData,
  tournamentYearDemands: TournamentYearDemand[],
  postCreate: (ts: TournamentSubmission) => void,
  apiKey: ApiKey
}

function CompeteTournamentManager(props: CompeteTournamentManagerProps) {
  let [showCreateTournamentSubmissionModal, setShowCreateTournamentSubmissionModal] = React.useState(false)

  // render the demand for the latest year
  let myYearDemands = props.tournamentYearDemands.filter(yd => yd.userId === props.apiKey.creatorUserId);

  // sort ascending
  myYearDemands.sort((a, b) => a.year - b.year);

  // get last one (most recent)
  let demand = myYearDemands[myYearDemands.length - 1]

  if (demand === undefined) {
    return <h5>Waiting for game to start...</h5>
  }


  return <Card>
    <Card.Body>
      <Card.Title>Year {demand.year + 1}</Card.Title>
      <Card.Text>
        You've calculated that the total demand for the
        product in the system is <b>{props.tournamentData.tournament.demandYintercept}</b>.
      </Card.Text>
      <Card.Text>
        If everyone sells too much, then the price per barrel will decline.
        However, if you sell a lot, and your competitors only a little, then you can secure a lion's share of the profits!
      </Card.Text>
      <Button onClick={() => setShowCreateTournamentSubmissionModal(true)}>
        Choose Production Amount
      </Button>
    </Card.Body>
    <DisplayModal
      title="Decide Production"
      show={showCreateTournamentSubmissionModal}
      onClose={() => setShowCreateTournamentSubmissionModal(false)}
    >
      <CreateTournamentSubmission
        apiKey={props.apiKey}
        tournamentData={props.tournamentData}
        postSubmit={ts => {
          setShowCreateTournamentSubmissionModal(false);
          props.postCreate(ts);
        }}
      />
    </DisplayModal>
  </Card>

}

export default CompeteTournamentManager;
