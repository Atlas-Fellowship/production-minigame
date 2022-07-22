import React from 'react';
import { Table } from 'react-bootstrap';
import update from 'immutability-helper';
import { ApiKey } from '@innexgo/frontend-auth-api';
import { Link, AddButton, DisplayModal } from '@innexgo/common-react-components';
import { TournamentSubmission } from '../utils/api';
import { ViewUser } from './ViewData';

import { Eye as ViewIcon } from 'react-bootstrap-icons';
import assert from 'assert';

type ManageTournamentSubmissionsCompeteProps = {
  // only discuss our own submissions
  tournamentSubmissions: TournamentSubmission[],
  setTournamentSubmissions: TournamentSubmission[],
  apiKey: ApiKey,
}

function ManageTournamentSubmissionsCompete(props: ManageTournamentSubmissionsCompeteProps) {
  const submissions = props.tournamentSubmissions.map(x => x);

  const map = new Map<number, TournamentSubmission[]>();
  for (const s of submissions) {
    const v = map.get(s.year);
    if (v) {
      v.push(s);
    } else {
      map.set(s.year, [s]);
    }
  }

  const map2 = new Map<number, {
    totalProduction: number,
    mySubmission: TournamentSubmission
    myProfit: number
  }>();


  for (const [year, tss] of map) {
    let totalProduction = 0;
    let mySubmission;
    for (const ts of tss) {
      totalProduction += ts.amount;
      if (ts.creatorUserId === props.apiKey.creatorUserId) {
        mySubmission = ts;
      }
    }
    assert(mySubmission !== undefined);

    const profitPerUnit = 2200 - totalProduction - 1000;

    const myProfit = mySubmission.amount * profitPerUnit;

    map2.set(year, { totalProduction, mySubmission, myProfit });
  }


  return <Table hover bordered>
    <thead>
      <tr>
        <th>Year</th>
        <th>Total Demand</th>
        <th>Profit per Megabarrel</th>
        <th>Production</th>
      </tr>
    </thead>
    <tbody>
      {submissions.length === 0
        ? <tr><td className="text-center" colSpan={4}>No Years</td></tr>
        : <> </>
      }
      {Array.from(map, ([year, yearlySubmissions]) =>
        <tr key={year}>
          <td>Year {year + 1}</td>
          <td>2200</td>
          <td>2200</td>
          <td>
            <Table hover bordered>
              <thead>
                <tr>
                  <th>Rank</th>
                  <th>Amount</th>
                </tr>
              </thead>
              <tbody>
                {yearlySubmissions.length === 0
                  ? <tr><td className="text-center" colSpan={2}>No Submissions</td></tr>
                  : null
                }
                {yearlySubmissions.map((s, i) =>
                  <tr
                    style={{
                      backgroundColor: s.creatorUserId === props.apiKey.creatorUserId
                        ? "#FFFF00"
                        : undefined
                    }}
                  >
                    <td>{i + i}</td>
                    <td>{s.amount}</td>
                  </tr>
                )}
              </tbody>
            </Table>
          </td>
        </tr>
      )}
    </tbody>
  </Table>
}

export default ManageTournamentSubmissionsCompete;
