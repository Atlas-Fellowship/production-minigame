import { Card, Container, Form, Table } from 'react-bootstrap';
import DashboardLayout from '../components/DashboardLayout';
import { Loader, WidgetWrapper, Link, Section } from '@innexgo/common-react-components';
import ErrorMessage from '../components/ErrorMessage';
import CompeteTournamentManager from '../components/CompeteTournamentManager';

import update from 'immutability-helper';

import { unwrap, getFirstOr } from '@innexgo/frontend-common';

import format from "date-fns/format";

import { Async, AsyncProps } from 'react-async';
import { TournamentData, tournamentDataView, TournamentSubmission, tournamentSubmissionView, TournamentYear, TournamentYearDemand, tournamentYearDemandView, tournamentYearView } from '../utils/api';
import { ApiKey } from '@innexgo/frontend-auth-api';
import { AuthenticatedComponentProps } from '@innexgo/auth-react-components';
import ManageTournamentSubmissionsOverview from '../components/ManageTournamentSubmissionOverview';

type TournamentCompetePageData = {
  tournamentData: TournamentData,
  tournamentSubmissions: TournamentSubmission[],
  tournamentYears: TournamentYear[],
  tournamentYearDemands: TournamentYearDemand[],
}

const loadTournamentCompetePage = async (props: AsyncProps<TournamentCompetePageData>): Promise<TournamentCompetePageData> => {
  const tournamentData = await tournamentDataView({
    tournamentId: [props.tournamentId],
    onlyRecent: true,
    apiKey: props.apiKey.key
  })
    .then(unwrap)
    .then(x => getFirstOr(x, "NOT_FOUND"))
    .then(unwrap);

  const tournamentSubmissions = await tournamentSubmissionView({
    tournamentId: [props.tournamentId],
    apiKey: props.apiKey.key
  })
    .then(unwrap);

  const tournamentYears = await tournamentYearView({
    tournamentId: [props.tournamentId],
    apiKey: props.apiKey.key,
    onlyRecent: false,
  })
    .then(unwrap);

  const tournamentYearDemands = await tournamentYearDemandView({
    tournamentId: [props.tournamentId],
    apiKey: props.apiKey.key,
  })
    .then(unwrap);

  return {
    tournamentData,
    tournamentYears,
    tournamentYearDemands,
    tournamentSubmissions,
  };
}

function TournamentCompetePage(props: AuthenticatedComponentProps) {
  const tournamentId = parseInt(new URLSearchParams(window.location.search).get("tournamentId") ?? "");

  return (
    <DashboardLayout {...props}>
      <Container fluid className="py-4 px-4">
        <Async promiseFn={loadTournamentCompetePage} tournamentId={tournamentId} apiKey={props.apiKey}>{
          ({ setData }) => <>
            <Async.Pending><Loader /></Async.Pending>
            <Async.Rejected>{e => <ErrorMessage error={e} />}</Async.Rejected>
            <Async.Fulfilled<TournamentCompetePageData>>{data => <>
              <Section name="Competition History" id="overview">
                <ManageTournamentSubmissionsOverview
                  tournamentData={data.tournamentData}
                  tournamentYears={data.tournamentYears}
                  tournamentYearDemands={data.tournamentYearDemands}
                  tournamentSubmissions={data.tournamentSubmissions}
                  apiKey={props.apiKey}
                  adminView={false}
                  onlyShowYou={true}
                />
              </Section>
              <Section name="Compete" id="compete">
                <CompeteTournamentManager
                  tournamentData={data.tournamentData}
                  tournamentYearDemands={data.tournamentYearDemands}
                  postCreate={ts => setData(update(data, { tournamentSubmissions: { $push: [ts] } }))}
                  apiKey={props.apiKey}
                />
              </Section>
            </>}
            </Async.Fulfilled>
          </>}
        </Async>
      </Container>
    </DashboardLayout>
  )
}


export default TournamentCompetePage;
