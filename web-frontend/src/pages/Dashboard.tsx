import { Card, Row, Container, Col, Button } from 'react-bootstrap';
import { Async, AsyncProps } from 'react-async';
import update from 'immutability-helper';
import { Section, Loader, AddButton, DisplayModal, } from '@innexgo/common-react-components';
import ErrorMessage from '../components/ErrorMessage';
import ExternalLayout from '../components/ExternalLayout';

import { unwrap } from '@innexgo/frontend-common';
import format from 'date-fns/format';
import formatDistance from 'date-fns/formatDistance';
import AuthenticatedComponentProps from '@innexgo/auth-react-components/lib/components/AuthenticatedComponentProps';

import { TournamentData, tournamentDataView, TournamentMembership, tournamentMembershipView } from '../utils/api';
import { DefaultSidebarLayout } from '@innexgo/auth-react-components';
import DashboardLayout from '../components/DashboardLayout';
import React from 'react';
import CreateTournament from '../components/CreateTournament';
import CreateTournamentMembership from '../components/CreateTournamentMembership';

type Data = {
  // all tournaments
  tournamentData: TournamentData[],
  // all tournament memberships
  tournamentMemberships: TournamentMembership[],
}

const loadData = async (props: AsyncProps<Data>) => {
  const tournamentData =
    await tournamentDataView({
      active: true,
      onlyRecent: true,
      apiKey: props.apiKey.key
    })
      .then(unwrap);

  const tournamentMemberships =
    await tournamentMembershipView({
      creatorUserId: [props.apiKey.creatorUserId],
      onlyRecent: true,
      apiKey: props.apiKey.key
    })
      .then(unwrap);

  return {
    tournamentData,
    tournamentMemberships
  }
}


type ResourceCardProps = {
  className?: string,
  title: string,
  subtitle: string,
  text: string,
  href: string
}

function ResourceCard(props: ResourceCardProps) {
  return (
    <Card style={{ width: '15rem' }} className={props.className}>
      <Card.Body>
        <Card.Title>{props.title}</Card.Title>
        <Card.Subtitle className="text-muted">{props.subtitle}</Card.Subtitle>
        <Card.Text>{props.text}</Card.Text>
        <Card.Link href={props.href} className="stretched-link" />
      </Card.Body>
    </Card>
  )
}


type ActionCardProps = {
  className?: string,
  title: string,
  subtitle: string,
  text: string,
  onClick: () => void,
  buttonText: string
}

function ActionCard(props: ActionCardProps) {
  return (
    <Card style={{ width: '15rem' }} className={props.className}>
      <Card.Body>
        <Card.Title>{props.title}</Card.Title>
        <Card.Subtitle className="text-muted">{props.subtitle}</Card.Subtitle>
        <Card.Text>{props.text}</Card.Text>
        <Button onClick={props.onClick}>{props.buttonText}</Button>
      </Card.Body>
    </Card>
  )
}

type AddNewCardProps = {
  className?: string,
  setShow: (a: boolean) => void,
};

const AddNewCard = (props: AddNewCardProps) =>
  <div className={props.className} style={{ width: "15rem", height: "100%" }}>
    <AddButton onClick={() => props.setShow(true)} />
  </div>


function Dashboard(props: AuthenticatedComponentProps) {

  const [showNewTournamentModal, setShowNewTournamentModal] = React.useState(false);
  const [showJoinTournamentModal, setShowJoinTournamentModal] = React.useState<TournamentData | null>(null);


  return <DashboardLayout {...props}>
    <Container fluid className="py-4 px-4">
      <Section id="tournaments" name="My Tournaments">
        <Async promiseFn={loadData} apiKey={props.apiKey}>
          {({ setData }) => <>
            <Async.Pending><Loader /></Async.Pending>
            <Async.Rejected>
              {e => <ErrorMessage error={e} />}
            </Async.Rejected>
            <Async.Fulfilled<Data>>{d =>
              <div className="d-flex flex-wrap">
                {
                  // tournaments you own
                  d.tournamentData.filter(a => a.creatorUserId === props.apiKey.creatorUserId).map(a =>
                    <ResourceCard
                      key={a.tournamentDataId}
                      className="m-2"
                      title={a.title}
                      subtitle="ADMIN"
                      text={`Created ${format(a.creationTime, "MMM d, Y")}`}
                      href={`/tournament_admin?tournamentId=${a.tournament.tournamentId}`}
                    />
                  )
                }
                {
                  // tournaments you're a member of 
                  d.tournamentData
                    .filter(a =>
                      d.tournamentMemberships.some(m => m.tournament.tournamentId == a.tournament.tournamentId)
                    )
                    .map(a =>
                      <ResourceCard
                        key={a.tournamentDataId}
                        className="m-2"
                        title={a.title}
                        subtitle="PLAYER"
                        text={`Created ${format(a.creationTime, "MMM d, Y")}`}
                        href={`/tournament_overview?tournamentId=${a.tournament.tournamentId}`}
                      />
                    )
                }
                {
                  // show new tournaments of which you are not a member
                  d.tournamentData
                    .filter(a =>
                      !d.tournamentMemberships.some(m => m.tournament.tournamentId == a.tournament.tournamentId)
                      && a.creatorUserId !== props.apiKey.creatorUserId
                    )
                    .map(a =>
                      <ActionCard
                        key={a.tournamentDataId}
                        className="m-2"
                        title={a.title}
                        subtitle=""
                        text={`Created ${format(a.creationTime, "MMM d, Y")}`}
                        buttonText="Join!"
                        onClick={() => setShowJoinTournamentModal(a)}
                      />
                    )
                }
                <AddNewCard className="m-2" setShow={setShowNewTournamentModal} />
                <DisplayModal
                  title="Create New Tournament"
                  show={showNewTournamentModal}
                  onClose={() => setShowNewTournamentModal(false)}
                >
                  <CreateTournament apiKey={props.apiKey}
                    postSubmit={(td) => {
                      setShowNewTournamentModal(false);
                      setData(update(d, { tournamentData: { $push: [td] } }));
                    }}
                  />
                </DisplayModal>
                {showJoinTournamentModal === null
                  ? null
                  : <DisplayModal
                    title="Join Tournament"
                    show={showJoinTournamentModal !== null}
                    onClose={() => setShowJoinTournamentModal(null)}
                  >
                    <CreateTournamentMembership apiKey={props.apiKey}
                      tournamentData={showJoinTournamentModal}
                      postSubmit={(tm) => {
                        setShowJoinTournamentModal(null);
                        setData(update(d, { tournamentMemberships: { $push: [tm] } }));
                      }}
                    />
                  </DisplayModal>
                }
              </div>}
            </Async.Fulfilled>
          </>}
        </Async>
      </Section>
    </Container>
  </DashboardLayout>
}

export default Dashboard;
