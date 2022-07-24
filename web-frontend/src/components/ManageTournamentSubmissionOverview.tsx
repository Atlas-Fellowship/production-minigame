import React from 'react';
import { Table } from 'react-bootstrap';
import update from 'immutability-helper';
import { ApiKey } from '@innexgo/frontend-auth-api';
import { Link, AddButton, DisplayModal } from '@innexgo/common-react-components';
import { TournamentData, TournamentSubmission, TournamentYear, TournamentYearDemand, tournamentYearView } from '../utils/api';
import { ViewUser } from './ViewData';
import Plot from 'react-plotly.js';

import { Eye as ViewIcon } from 'react-bootstrap-icons';
import { assert, sum } from '../utils/utils';

type ManageTournamentSubmissionsOverviewProps = {
  tournamentData: TournamentData,
  tournamentSubmissions: TournamentSubmission[],
  tournamentYears: TournamentYear[],
  tournamentYearDemands: TournamentYearDemand[],
  adminView: boolean,
  apiKey: ApiKey,
}

// only get the latest tournament submission per year per user
function getRecentSubmissions(tss: TournamentSubmission[]) {
  const map = new Map<string, TournamentSubmission[]>();
  for (const ts of tss) {
    const id = JSON.stringify([ts.creatorUserId, ts.year]);
    const result = map.get(id);
    if (result === undefined) {
      map.set(id, [ts]);
    } else {
      result.push(ts);
    }
  }

  const results: TournamentSubmission[] = []

  // select one with max id
  for (const [, value] of map) {
    let maxIdTS: null | TournamentSubmission = null;
    for (const ts of value) {
      if (maxIdTS === null || ts.tournamentSubmissionId > maxIdTS.tournamentSubmissionId) {
        maxIdTS = ts;
      }
    }
    if (maxIdTS) {
      results.push(maxIdTS);
    }
  }

  return results;
}

type DemandCurvePlotProps = {
  xIntercept: number,
  yIntercept: number,
  quantity: number,
  price: number,
}

function DemandCurvePlot(props: DemandCurvePlotProps) {
  const { yIntercept, xIntercept, quantity, price } = props;

  return <Plot
    className="mx-2"
    data={[
      {
        type: 'scatter',
        name: "Demand Curve",
        x: [0, xIntercept],
        y: [yIntercept, 0],
        mode: 'lines+markers',
        marker: { color: 'red' },
        hovertemplate: '<b>Quantity</b>: %{x}<br><b>Price per Unit</b>: $%{y}<extra></extra>'
      },
      {
        type: 'scatter',
        name: "Production",
        x: [quantity],
        y: [price],
        mode: 'markers',
        marker: { color: 'purple' },
        hovertemplate: '<b>Quantity</b>: %{x}<br><b>Price per Unit</b>: $%{y}<extra></extra>'
      }
    ]}
    layout={{
      hovermode: 'closest',
      xaxis: { title: { text: 'Quantity (Units)', }, fixedrange: true },
      yaxis: { title: { text: 'Price Per Unit ($)', }, fixedrange: true },
      width: 400,
      height: 200,
      margin: { b: 40, l: 50, r: 10, t: 10 },
    }}
    config={{
      displayModeBar: false
    }}
  />
}


function ManageTournamentSubmissionsOverview(props: ManageTournamentSubmissionsOverviewProps) {
  const submissions = getRecentSubmissions(props.tournamentSubmissions);
  const years = props.tournamentYears.map(x => x);
  const demands = props.tournamentYearDemands.map(x => x);

  // sort by year ascending
  years.sort((a, b) => a.currentYear - b.currentYear);

  const balanceMap = new Map<number, number>();

  const data = years.map(y => {
    const subs = submissions.filter(s => s.year === y.currentYear);
    // sort by amount descending
    subs.sort((a, b) => b.amount - a.amount);

    const yearDemands = demands.filter(yd => yd.year === y.currentYear);

    const totalIncentive = sum(yearDemands.map(yd => yd.demand));
    const totalProduction = sum(subs.map(s => s.amount));

    const { demandXintercept: xIntercept, demandYintercept: yIntercept } = props.tournamentData.tournament;

    // slope
    let m = -yIntercept / xIntercept;
    // y intercept
    let b = yIntercept + totalIncentive;

    // demandCurveProfit
    const revenuePerUnit = Math.max(m * totalProduction + b, 0);
    const profitPerUnit = revenuePerUnit - props.tournamentData.tournament.costPerUnit;

    const subData = subs.map((s, i) => {
        // submission profit
        const profit = s.amount * profitPerUnit;
        const result = balanceMap.get(s.creatorUserId);
        let balance = 0;
        if(result === undefined) {
            balanceMap.set(s.creatorUserId, profit);
            balance = profit;
        } else {
            balance = profit + result
            balanceMap.set(s.creatorUserId, balance);
        }

        return {
            s,
            i,
            profit,
            balance
        };
    });

    return { y, subData, totalProduction, revenuePerUnit, profitPerUnit, xIntercept, yIntercept }
  });


  return <Table hover bordered>
    <thead>
      <tr>
        <th>Year</th>
        <th>Production</th>
        <th>Demand Curve</th> {/* Also need to put down the profit per unit in this zone */}
        <th>Profits and Balance</th>
      </tr>
    </thead>
    <tbody>
      {data.length === 0
        ? <tr><td className="text-center" colSpan={4}>No Years</td></tr>
        : <> </>
      }
      {data.map(d =>
        <tr key={d.y.currentYear}>
          <td>Year {d.y.currentYear + 1}</td>
          <td>
            <Table hover bordered>
              <thead>
                <tr>
                  <th>Rank</th>
                  <th>Player</th>
                  <th>Production</th>
                </tr>
              </thead>
              <tbody>{d.subData.length === 0
                ? <tr><td className="text-center" colSpan={4}>No Submissions</td></tr>
                : <> </>
              }
                {d.subData.map(({s, i}) =>
                  <tr key={i} >
                    <td>{i + 1}</td>
                    <td>{
                      props.adminView
                        ? <ViewUser userId={s.creatorUserId} apiKey={props.apiKey} expanded={false} />
                        : s.creatorUserId === props.apiKey.creatorUserId
                          ? <b style={{ backgroundColor: "yellow" }}>You</b>
                          : <b> (hidden) </b>
                    }</td>
                    <td>{
                      s.autogenerated
                        ? `${s.amount} (auto)`
                        : s.amount
                    }</td>
                  </tr>
                )}</tbody>
            </Table>
            <p>Total Production: <b>{d.totalProduction} units</b></p>
          </td>
          <td>
            <DemandCurvePlot xIntercept={d.xIntercept} yIntercept={d.yIntercept} quantity={d.totalProduction} price={d.revenuePerUnit} />
            <p>Sale price per unit was: <b>${d.revenuePerUnit}</b></p>
            <p>Each unit cost ${props.tournamentData.tournament.costPerUnit}, so profit per unit was: <b>${d.profitPerUnit}</b></p>
          </td>
          <td>
            <Table hover bordered>
              <thead>
                <tr>
                  <th>Player</th>
                  <th>Profit</th>
                  <th>Balance</th>
                </tr>
              </thead>
              <tbody>{d.subData.length === 0
                ? <tr><td className="text-center" colSpan={4}>No Submissions</td></tr>
                : <> </>
              }
                {d.subData.map(({s, i, profit, balance}) =>
                  <tr key={i} >
                    <td>{
                      props.adminView
                        ? <ViewUser userId={s.creatorUserId} apiKey={props.apiKey} expanded={false} />
                        : s.creatorUserId === props.apiKey.creatorUserId
                          ? <b style={{ backgroundColor: "yellow" }}>You</b>
                          : <b> (hidden) </b>
                    }</td>
                    <td>{s.amount} units*${d.profitPerUnit}/unit = <b>${profit}</b></td>
                    <td>{balance}</td>
                  </tr>
                )}</tbody>
            </Table>
          </td>
        </tr>
      )}
    </tbody>
  </Table>
}

export default ManageTournamentSubmissionsOverview;
